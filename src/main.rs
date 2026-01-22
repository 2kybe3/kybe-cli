mod config;

use std::sync::Arc;

use anyhow::Context;
use clap::Parser;
use parking_lot::Mutex;

use crate::config::types::Config;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Allows you to specify a custom config path
    #[arg(long, value_hint = clap::ValueHint::FilePath)]
    config: Option<std::path::PathBuf>,

    /// Allows you to edit the config with your prefered editor
    #[arg(long)]
    edit_config: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let cfg = Arc::new(Mutex::new(Config::load(args.config)?));

    if args.edit_config {
        let mut cfg_guard = cfg.lock();

        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".into());

        if let Some(path) = &cfg_guard.path {
            let status = std::process::Command::new(editor)
                .arg(path)
                .status()
                .context("failed to open editor")?;

            if !status.success() {
                eprintln!("Editor exited with non-zero status");
            }

            cfg_guard.reload()?;
        } else {
            eprintln!("Config has no path associated; cannot edit");
        }
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
        println!("{:?}", *cfg_lock);
    }

    // 2. parse the argument -> decides what action to run
    // should call commands/smth to run the command and maybe login once thats implemented

    cfg.lock().save()?;

    Ok(())
}
