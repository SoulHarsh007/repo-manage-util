use clap::Parser;

#[derive(Parser, PartialEq, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(subcommand_negates_reqs = true)]
pub(crate) struct Cli {
    /// Profile to use from the configuration file
    #[arg(global = true, short, long)]
    profile: Option<String>,

    /// Profile to use from the configuration file
    #[arg(global = true, short, long)]
    from: Option<String>,
    /// Profile to use from the configuration file
    #[arg(global = true, short, long)]
    to: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser, PartialEq, Debug)]
pub(crate) struct SingleProfileCli {
    /// Profile to use from the configuration file
    #[clap(from_global)]
    pub profile: String,
}

#[derive(Parser, PartialEq, Debug)]
pub(crate) struct FromToProfileCli {
    /// Profile to use from the configuration file (for move-pkgs) FROM repo
    #[clap(from_global)]
    pub from: String,
    /// Profile to use from the configuration file (for move-pkgs) TO repo
    #[clap(from_global)]
    pub to: String,
}

#[derive(Parser, PartialEq, Debug)]
pub(crate) enum Commands {
    /// Reset the repository
    Reset(SingleProfileCli),
    /// Update the repository
    Update(SingleProfileCli),
    /// Moves packages from current directory into the repository
    MovePkgsToRepo(SingleProfileCli),
    /// Moves packages from one repository to another repository
    MovePkgs(FromToProfileCli),
    /// Check if the packages are up-to-date
    IsPkgsUpToDate(SingleProfileCli),
    /// Cleans up the backup directory,
    /// removing the N amount of packages if configured to do so
    CleanupBackupDir(SingleProfileCli),
    // Check if we have only certain amount of debug packages in the debug repository
    // IsDebugPkgsOk, // ok maybe not implemented
}

#[cfg(test)]
mod tests {
    use crate::{Cli, Commands, FromToProfileCli, SingleProfileCli};

    use clap::Parser;

    #[test]
    fn single_profile_check() {
        assert_eq!(Cli::parse_from(["test", "reset", "--profile", "abcd"]), Cli {
            profile: Some("abcd".to_owned()),
            to: None,
            from: None,
            command: Commands::Reset(SingleProfileCli { profile: "abcd".to_owned() })
        });

        assert_eq!(Cli::parse_from(["test", "--profile", "abcd", "reset"]), Cli {
            profile: Some("abcd".to_owned()),
            to: None,
            from: None,
            command: Commands::Reset(SingleProfileCli { profile: "abcd".to_owned() })
        });
    }

    #[test]
    fn from_to_profile_check() {
        assert_eq!(
            Cli::parse_from(["test", "move-pkgs", "--to", "abcd", "--from", "dcba"]),
            Cli {
                profile: None,
                to: Some("abcd".to_owned()),
                from: Some("dcba".to_owned()),
                command: Commands::MovePkgs(FromToProfileCli {
                    to: "abcd".to_owned(),
                    from: "dcba".to_owned()
                })
            }
        );

        assert_eq!(
            Cli::parse_from(["test", "--to", "abcd", "--from", "dcba", "move-pkgs"]),
            Cli {
                profile: None,
                to: Some("abcd".to_owned()),
                from: Some("dcba".to_owned()),
                command: Commands::MovePkgs(FromToProfileCli {
                    to: "abcd".to_owned(),
                    from: "dcba".to_owned()
                })
            }
        );
    }
}
