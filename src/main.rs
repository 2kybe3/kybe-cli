mod config;

use std::sync::Arc;

use clap::Parser;
use parking_lot::Mutex;

use crate::config::types::Config;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, value_hint = clap::ValueHint::FilePath)]
    config: Option<std::path::PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let cfg = Arc::new(Mutex::new(Config::load(args.config)?));

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
