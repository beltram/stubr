use clap::Clap;

#[derive(Clap, Debug, PartialEq)]
pub enum Shell {
    /// generates cli completion file for bash
    Bash,
    /// generates cli completion file for zsh
    Zsh,
}