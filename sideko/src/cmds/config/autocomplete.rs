use crate::{
    cli::SidekoCli,
    result::{CliError, CliResult},
    styles::fmt_green,
};
use clap::{Args, CommandFactory};
use clap_complete::{generate, Shell};
use dirs::home_dir;
use inquire::Confirm;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Args)]
pub(crate) struct AutocompleteCommand {
    /// Ggnerate completions for the specified shell
    #[arg(long)]
    shell: Shell,
}

impl AutocompleteCommand {
    pub async fn handle(&self) -> CliResult<()> {
        let mut cmd = SidekoCli::command();
        let name = cmd.get_name().to_string();

        // Auto-install to appropriate location
        let (completion_path, rc_path) = self.get_shell_paths()?;

        // Ask for permission to modify files
        let message = format!(
            "this will:\n\
                     1. create completion script at: {}\n\
                     2. update shell configuration at: {}\n\n\
                     continue?",
            completion_path.display(),
            rc_path.display()
        );

        let confirm = Confirm::new(&message)
            .with_default(true)
            .prompt()
            .map_err(|err| CliError::Inquire {
                err,
                override_msg: None,
            })?;

        if !confirm {
            return Ok(());
        }

        // Create completion directory if needed
        if let Some(parent) = completion_path.parent() {
            fs::create_dir_all(parent).map_err(|err| CliError::Io {
                err,
                override_msg: Some("failed to create completion directory".to_string()),
            })?;
        }

        // Generate completion script
        let mut file = File::create(&completion_path).map_err(|err| CliError::Io {
            err,
            override_msg: Some("failed to create completion file".to_string()),
        })?;
        generate(self.shell, &mut cmd, &name, &mut file);

        // Update shell RC file
        self.update_rc_file(&rc_path, &completion_path)?;

        println!(
            "{} installed {} completions for {}",
            fmt_green("✓"),
            self.shell,
            name
        );
        println!(
            "{} saved completion script: {}",
            fmt_green("✓"),
            completion_path.display()
        );
        println!(
            "{} saved updated RC file: {}",
            fmt_green("✓"),
            rc_path.display()
        );

        Ok(())
    }

    fn get_shell_paths(&self) -> CliResult<(PathBuf, PathBuf)> {
        let home = home_dir().ok_or(CliError::General {
            msg: "could not find home directory".to_string(),
            debug: None,
        })?;

        let (completion_path, rc_file) = match self.shell {
            Shell::Bash => (
                home.join(".bash_completion.d")
                    .join(SidekoCli::command().get_name()),
                ".bashrc",
            ),
            Shell::Zsh => (
                home.join(".zfunc")
                    .join(format!("_{}", SidekoCli::command().get_name())),
                ".zshrc",
            ),
            Shell::Fish => (
                home.join(".config/fish/completions")
                    .join(format!("{}.fish", SidekoCli::command().get_name())),
                "config.fish",
            ),
            _ => {
                return Err(CliError::General {
                    msg: format!("Unsupported shell: {:?}", self.shell),
                    debug: None,
                })
            }
        };

        let rc_path = home.join(rc_file);

        Ok((completion_path, rc_path))
    }

    fn update_rc_file(&self, rc_path: &PathBuf, completion_path: &Path) -> CliResult<()> {
        // Create RC file if it doesn't exist
        if !rc_path.exists() {
            let confirm = Confirm::new(&format!(
                "rc file {} does not exist. create it?",
                rc_path.display()
            ))
            .with_default(true)
            .prompt()
            .map_err(|err| CliError::Inquire {
                err,
                override_msg: None,
            })?;

            if !confirm {
                return Ok(());
            }

            File::create(rc_path).map_err(|err| CliError::Io {
                err,
                override_msg: Some("failed to create rc file".to_string()),
            })?;
        }

        let content = fs::read_to_string(rc_path).map_err(|err| CliError::Io {
            err,
            override_msg: Some("failed to read rc file".to_string()),
        })?;

        // Generate appropriate source command
        let source_line = match self.shell {
            Shell::Bash => format!(
                "\n# added by sideko\n[[ -f {} ]] && source {}\n",
                completion_path.display(),
                completion_path.display()
            ),
            Shell::Zsh => {
                "\n# added by sideko\nfpath=(~/.zfunc $fpath)\nautoload -Uz compinit && compinit\n"
                    .to_string()
            }
            Shell::Fish => String::new(), // Fish automatically loads from ~/.config/fish/completions
            _ => return Ok(()),
        };

        // Only append if not already present
        if !content.contains(&source_line) {
            let mut file = fs::OpenOptions::new()
                .append(true)
                .open(rc_path)
                .map_err(|err| CliError::Io {
                    err,
                    override_msg: Some("failed to open rc file for writing".to_string()),
                })?;
            file.write_all(source_line.as_bytes())
                .map_err(|err| CliError::Io {
                    err,
                    override_msg: Some("failed to update rc file".to_string()),
                })?;
        }

        Ok(())
    }
}
