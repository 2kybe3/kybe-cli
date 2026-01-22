mod config;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, value_hint = clap::ValueHint::FilePath)]
    config: Option<std::path::PathBuf>,
}

fn main() {
    let _args = Args::parse();
    // TODO; make smth like the following work
    // let cfg = Config::load(args.config);

    // 2. parse the argument -> decides what action to run
    // should call commands/smth to run the command and maybe login once thats implemented
}
