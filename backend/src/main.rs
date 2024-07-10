use mtrack::Config;

use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Setting up application failed: {:?}", err);
        process::exit(1);
    });

    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(err) => {
            eprintln!("Setting up tokio runtime failed: {:?}", err);
            process::exit(1);
        }
    };

    if let Err(err) = rt.block_on(mtrack::run(config)) {
        eprintln!("Running application failed: {:?}", err);
        process::exit(1);
    }

    process::exit(0);
}
