use std::{fs::canonicalize, path::PathBuf};
use std::fs::{create_dir, create_dir_all, remove_dir_all};

use cargo::{
    Config,
    core::{Dependency, Package, Resolve, SourceId, Workspace},
    ops::{load_pkg_lockfile, read_package},
    util::hex,
};
use fs_extra::dir::CopyOptions;
use fs_extra::error::{Error as CopyError, ErrorKind};

pub fn consumer() {
    eprintln!("+++");
    let consumer = StubrConsumer::new();
    consumer.copy_stubs();
    eprintln!("+++");
}

struct StubrConsumer {
    config: Config,
    manifest_path: PathBuf,
    package: Package,
}

impl StubrConsumer {
    const STUBS_DIR: &'static str = "stubs";
    const IMPORT_DIR: &'static str = "stubr";

    fn new() -> Self {
        let config = Config::default().unwrap();
        let cwd = canonicalize(config.cwd()).unwrap();
        let manifest_path = cwd.join("Cargo.toml");
        let source_id = SourceId::for_path(&cwd).unwrap();
        let package = read_package(&manifest_path, source_id, &config).unwrap().0;
        Self { config, manifest_path, package }
    }

    fn copy_stubs(&self) {
        let output_dir = self.workspace().target_dir();
        let output_dir = output_dir
            .join(Self::IMPORT_DIR)
            .join(self.package.name())
            .into_path_unlocked();
        if !output_dir.exists() { create_dir_all(&output_dir).unwrap(); }
        self.find_all_stubs().iter()
            .for_each(|(name, paths)| {
                let target = output_dir.join(name);
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

    fn find_all_stubs(&self) -> Vec<(String, Vec<PathBuf>)> {
        self.build_dependencies().iter()
            .filter_map(|d| {
                self.src_path(d)
                    .map(|p| (d.package_name().to_string(), self.find_stubs(p)))
            })
            .filter(|(_, files)| !files.is_empty())
            .collect()
    }

    fn src_path(&self, dep: &Dependency) -> Option<PathBuf> {
        dep.source_id().local_path()
            .or_else(|| self.resolve_remote_src_path(dep))
    }

    fn resolve_remote_src_path(&self, dep: &Dependency) -> Option<PathBuf> {
        self.resolve_package().iter()
            .find(|it| it.name() == dep.package_name())
            .map(|pkg| {
                let source = dep.source_id();
                let hash = hex::short_hash(&source);
                let host = source.url().host_str().unwrap();
                let part = format!("{}-{}", host, hash);
                let id = format!("{}-{}", pkg.name(), pkg.version());
                self.config.home()
                    .join("registry")
                    .join("src")
                    .join(part)
                    .join(id)
                    .into_path_unlocked()
            })
    }

    fn build_dependencies(&self) -> Vec<Dependency> {
        self.package.dependencies().into_iter()
            .filter(|d| d.is_build())
            .map(|it| it.to_owned())
            .collect()
    }

    fn find_stubs(&self, path: PathBuf) -> Vec<PathBuf> {
        path.join(Self::STUBS_DIR).read_dir().ok()
            .map(|dir| dir.map(|it| it.unwrap().path()).collect())
            .unwrap_or_default()
    }

    fn resolve_package(&self) -> Resolve {
        load_pkg_lockfile(&self.workspace()).unwrap().unwrap()
    }

    fn workspace(&self) -> Workspace<'_> {
        Workspace::new(&self.manifest_path, &self.config).unwrap()
    }
}