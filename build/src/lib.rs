#![doc = include_str ! ("../README.md")]

use std::{
    fs::{canonicalize, create_dir, create_dir_all, remove_dir_all},
    path::PathBuf,
};

use cargo::{
    Config,
    core::{Dependency, Package, Resolve, SourceId, Workspace},
    ops::{load_pkg_lockfile, read_package},
    util::hex,
};
use fs_extra::{
    dir::CopyOptions,
    error::{Error as CopyError, ErrorKind},
};

/// This introspects package build dependencies and extracts json stubs files
/// located under a 'stubs' directory at package root into 'target/stubr/<consumer-name>/<producer-name>'
///
/// # Example
///
/// In your `build.rs` build script
///
/// ```no_run
/// # #[allow(clippy::needless_doctest_main)]
/// fn main() { stubr_build::stubr_consumer() }
/// ```
pub fn stubr_consumer() {
    StubrConsumer::new()
        .expect("Failed initializing stubr build plugin")
        .copy_stubs();
    println!("cargo:rerun-if-changed=.");
}

struct StubrConsumer {
    config: Config,
    manifest_path: PathBuf,
    package: Package,
}

impl StubrConsumer {
    const STUBS_DIR: &'static str = "stubs";
    const IMPORT_DIR: &'static str = "stubr";

    fn new() -> anyhow::Result<Self> {
        let config = Config::default()?;
        let cwd = canonicalize(config.cwd())?;
        let manifest_path = cwd.join("Cargo.toml");
        let source_id = SourceId::for_path(&cwd)?;
        let package = read_package(&manifest_path, source_id, &config)?.0;
        Ok(Self { config, manifest_path, package })
    }

    fn copy_stubs(&self) {
        let output_dir = self.workspace().target_dir();
        let output_dir = output_dir
            .join(Self::IMPORT_DIR)
            .join(self.package.name())
            .into_path_unlocked();
        if !output_dir.exists() { create_dir_all(&output_dir).unwrap(); }
        self.find_all_stubs()
            .for_each(|(name, paths)| {
                let target = output_dir.join(&name);
                if !target.exists() { create_dir_all(&target).unwrap(); } else {
                    remove_dir_all(&target).unwrap();
                    create_dir(&target).unwrap();
                }
                match fs_extra::copy_items(&paths, target, &CopyOptions::default()) {
                    Ok(_) => {}
                    Err(CopyError { kind, .. }) => {
                        if let ErrorKind::AlreadyExists = kind {} else {
                            panic!("Failed copying stubs from {} to target dir", name)
                        }
                    }
                };
            })
    }

    fn find_all_stubs(&self) -> impl Iterator<Item=(String, Vec<PathBuf>)> + '_ {
        self.build_dependencies()
            .filter_map(move |d| {
                self.src_path(d)
                    .map(|p| (d.package_name().to_string(), self.find_stubs(p)))
            })
            .filter(|(_, files)| !files.is_empty())
    }

    fn src_path(&self, dep: &Dependency) -> Option<PathBuf> {
        dep.source_id().local_path()
            .or_else(|| self.resolve_remote_src_path(dep))
    }

    fn resolve_remote_src_path(&self, dep: &Dependency) -> Option<PathBuf> {
        self.resolve_package().iter()
            .find(|it| it.name() == dep.package_name())
            .map(|pkg| format!("{}-{}", pkg.name(), pkg.version()))
            .and_then(|id| {
                let source = dep.source_id();
                source.url().host_str().map(|host| {
                    let hash = hex::short_hash(&source);
                    let part = format!("{}-{}", host, hash);
                    self.config.home()
                        .join("registry")
                        .join("src")
                        .join(part)
                        .join(id)
                        .into_path_unlocked()
                })
            })
    }

    fn build_dependencies(&self) -> impl Iterator<Item=&Dependency> {
        self.package.dependencies().iter()
            .filter(|d| d.is_build())
    }

    fn find_stubs(&self, path: PathBuf) -> Vec<PathBuf> {
        path.join(Self::STUBS_DIR).read_dir().ok()
            .map(|dir| dir.map(|it| it.unwrap().path()).collect())
            .unwrap_or_default()
    }

    fn resolve_package(&self) -> Resolve {
        load_pkg_lockfile(&self.workspace()).ok()
            .flatten()
            .unwrap_or_else(|| panic!("Failed resolving package at {:?}", self.manifest_path))
    }

    fn workspace(&self) -> Workspace<'_> {
        Workspace::new(&self.manifest_path, &self.config)
            .unwrap_or_else(|_| panic!("Failed resolving workspace at {:?}", self.manifest_path))
    }
}