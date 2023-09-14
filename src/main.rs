mod modules;

use cucumber::{cli, World};

#[derive(cli::Args, Clone)]
pub struct Args {
    #[arg(default_values_t = ["main.feature".to_string()])]
    pub targets: Vec<String>,
    #[arg(long)]
    pub serial: bool,
}

#[derive(World, Debug, Default)]
struct Env;

#[tokio::main]
async fn main() {
    let opts = cli::Opts::<_, _, _, Args>::parsed();
    let args = opts.clone().custom;

    for target in args.targets {
        Env::cucumber().with_cli(opts.clone()).run(target).await;
    }
}
