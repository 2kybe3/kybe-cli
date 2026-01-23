mod config;

use std::{io, sync::Arc};

use anyhow::Context;
use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::{Generator, Shell, generate};
use parking_lot::Mutex;

use crate::config::types::Config;

#[derive(Parser, Debug)]
#[command(version, long_about = None, name = "kcli")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(subcommand_required = true, arg_required_else_help = true)]
    Config {
        #[command(subcommand)]
        command: Option<GenerateCompletionsCommands>,
    },
    GenerateCompletions {
        #[arg(short, long)]
        shell: Shell,
    },
}

#[derive(Subcommand, Debug)]
enum GenerateCompletionsCommands {
    Show,
    Edit,
}

fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    );
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let cfg = Arc::new(Mutex::new(Config::load()?));

    if let Some(Commands::GenerateCompletions { shell }) = cli.command {
        let mut cmd = Cli::command();
        eprintln!("Generating completion file for {shell:?}...");
        print_completions(shell, &mut cmd);
        std::process::exit(0);
    }

    if let Some(Commands::Config { command }) = cli.command {
        match command {
            Some(GenerateCompletionsCommands::Edit) => {
                let cfg_guard = cfg.lock();

                let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".into());

                let status = std::process::Command::new(editor)
                    .arg(&cfg_guard.user_file)
                    .status()
                    .context("failed to open editor")?;

                if !status.success() {
                    eprintln!("Editor exited with non-zero status");
                }
            }
            Some(GenerateCompletionsCommands::Show) => {
                let cfg_guard = cfg.lock();

                println!("{:#?}", *cfg_guard);
            }
            None => {}
        }

        std::process::exit(0);
    }

    {
        let cfg = Arc::clone(&cfg);
        ctrlc::set_handler(move || {
            if let Err(e) = cfg.lock().save() {
                eprintln!("Failed to save config: {e:?}");
            }
            std::process::exit(0);
        })?;
    }

    {
        let mut cfg_lock = cfg.lock();
        cfg_lock.generated.last_launch = Some(chrono::Utc::now());
    }

    // 2. parse the argument -> decides what action to run
    // should call commands/smth to run the command and maybe login once thats implemented

    cfg.lock().save()?;

    Ok(())
}
