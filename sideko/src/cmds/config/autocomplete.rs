use crate::{
    cli::SidekoCli,
    result::{CliError, CliResult},
};
use clap::{Args, CommandFactory};
use clap_complete::{generate, Shell};
use dirs::home_dir;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Args)]
pub(crate) struct AutocompleteCommand {
    /// Generate completions for the specified shell
    #[arg(long)]
    shell: Shell,

    /// Optional output path for the completion script
    #[arg(long, value_hint = clap::ValueHint::FilePath)]
    out_file: Option<PathBuf>,
}

impl AutocompleteCommand {
    pub async fn handle(&self) -> CliResult<()> {
        let mut cmd = SidekoCli::command();
        let name = cmd.get_name().to_string();

        match &self.out_file {
            Some(path) => {
                // Write to specified file
                let mut file = File::create(path).map_err(|err| CliError::Io {
                    err,
                    override_msg: Some("Failed to create output file".to_string()),
                })?;
                generate(self.shell, &mut cmd, &name, &mut file);
                println!("✓ Generated completion script at: {}", path.display());
            }
            None => {
                // Auto-install to appropriate location
                let (completion_path, rc_path) = self.get_shell_paths()?;

                // Create completion directory if needed
                if let Some(parent) = completion_path.parent() {
                    fs::create_dir_all(parent).map_err(|err| CliError::Io {
                        err,
                        override_msg: Some("Failed to create completion directory".to_string()),
                    })?;
                }

                // Generate completion script
                let mut file = File::create(&completion_path).map_err(|err| CliError::Io {
                    err,
                    override_msg: Some("Failed to create completion file".to_string()),
                })?;
                generate(self.shell, &mut cmd, &name, &mut file);

                // Update shell RC file
                self.update_rc_file(&rc_path, &completion_path)?;

                println!("✓ Installed {} completions for {}", self.shell, name);
                println!("  Completion script: {}", completion_path.display());
                println!("  Updated RC file: {}", rc_path.display());
            }
        }

        Ok(())
    }

    fn get_shell_paths(&self) -> CliResult<(PathBuf, PathBuf)> {
        let home = home_dir().ok_or(CliError::General {
            msg: "Could not find home directory".to_string(),
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
            File::create(rc_path).map_err(|err| CliError::Io {
                err,
                override_msg: Some("Failed to create RC file".to_string()),
            })?;
        }

        let content = fs::read_to_string(rc_path).map_err(|err| CliError::Io {
            err,
            override_msg: Some("Failed to read RC file".to_string()),
        })?;

        // Generate appropriate source command
        let source_line = match self.shell {
            Shell::Bash => format!(
                "\n# Added by sideko\n[[ -f {} ]] && source {}\n",
                completion_path.display(),
                completion_path.display()
            ),
            Shell::Zsh => {
                "\n# Added by sideko\nfpath=(~/.zfunc $fpath)\nautoload -Uz compinit && compinit\n"
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
                    override_msg: Some("Failed to open RC file for writing".to_string()),
                })?;
            file.write_all(source_line.as_bytes())
                .map_err(|err| CliError::Io {
                    err,
                    override_msg: Some("Failed to update RC file".to_string()),
                })?;
        }

        Ok(())
    }
}
