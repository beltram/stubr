use std::fs::create_dir_all;
use std::path::PathBuf;

use clap::{App, Clap, IntoApp};
use clap_generate::{generate_to, Generator, generators::{Bash, Zsh}};
use directories::UserDirs;

use crate::cli::Cli;

#[derive(Clap, Debug, Eq, PartialEq)]
pub enum Shell {
    /// generates cli completion file for bash
    Bash,
    /// generates cli completion file for zsh
    Zsh,
}

impl Shell {
    const ZSH_DIR: &'static str = "/usr/local/share/zsh/site-functions";
    const BASH_DIR: &'static str = ".bash_completion.d";

    pub(crate) fn generate_and_install(&self) {
        self.create_completion_for(Cli::into_app())
    }

    fn create_completion_for(&self, mut app: App) {
        match self {
            Shell::Bash => self.create_completion::<Bash>(&mut app),
            Shell::Zsh => self.create_completion::<Zsh>(&mut app),
        }
    }

    fn create_completion<G: Generator>(&self, app: &mut App) {
        let bin_name = app.get_name().to_string();
        let dir = self.completion_dir();
        generate_to::<G, _, _>(app, &bin_name, &dir)
            .expect("Failed generating completion file");
    }

    fn completion_dir(&self) -> PathBuf {
        let dir = match self {
            Shell::Zsh => PathBuf::from(Self::ZSH_DIR),
            Shell::Bash => Self::home().map(|it| it.join(Self::BASH_DIR)).expect("Could not find user home directory"),
        };
        if !dir.exists() {
            create_dir_all(&dir)
                .unwrap_or_else(|e| panic!("Failed creating non-existing directory {:?} because {:?}", &dir, e));
        }
        dir
    }

    fn home() -> Option<PathBuf> {
        UserDirs::new().map(|u| u.home_dir().to_path_buf())
    }
}