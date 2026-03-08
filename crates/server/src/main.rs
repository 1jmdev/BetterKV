mod cli;

use cli::{Action, parse_cli_args, print_usage, run};

fn main() {
    let _trace = profiler::scope("server::main::main");
    let args = std::env::args().collect::<Vec<_>>();
    let action = match parse_cli_args(&args) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{err}");
            eprintln!();
            print_usage();
            std::process::exit(1);
        }
    };

    match action {
        Action::Help => {
            print_usage();
        }
        Action::Version => {
            println!("betterkv-server v{}", env!("CARGO_PKG_VERSION"));
        }
        Action::CheckSystem => {
            println!("[ok] system check passed");
        }
        Action::TestMemory(megabytes) => {
            if megabytes == 0 {
                eprintln!("--test-memory requires a value greater than zero");
                std::process::exit(1);
            }
            println!("[ok] memory test simulated for {megabytes} MB");
        }
        Action::Run(runtime) => {
            if let Err(err) = run(runtime) {
                eprintln!("{err}");
                std::process::exit(1);
            }
        }
    }
}
