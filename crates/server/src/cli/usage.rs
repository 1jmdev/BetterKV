use std::path::Path;

pub(crate) fn print_usage() {
    let _trace = profiler::scope("server::main::print_usage");
    let bin = std::env::args()
        .next()
        .and_then(|value| {
            Path::new(&value)
                .file_name()
                .map(|v| v.to_string_lossy().to_string())
        })
        .unwrap_or_else(|| "betterkv-server".to_string());

    println!("Usage: {bin} [/path/to/betterkv.conf] [options] [-]");
    println!("       {bin} - (read config from stdin)");
    println!("       {bin} -v or --version");
    println!("       {bin} -h or --help");
    println!("       {bin} --test-memory <megabytes>");
    println!("       {bin} --check-system");
    println!();
    println!("Important directives:");
    println!("       --loglevel <trace|debug|info|warn|error>");
    println!("       --logfile <path|stdout>");
    println!("       --dir <snapshot-directory>");
    println!("       --dbfilename <snapshot-file>");
    println!("       --save <seconds> [changes]");
    println!("       --requirepass <password>");
    println!("       --user <name> <acl-rules...>");
    println!();
    println!("Examples:");
    println!("       {bin}");
    println!("       echo 'port 6380' | {bin} -");
    println!("       {bin} ./betterkv.conf --port 6379");
    println!("       {bin} --port 7777 --bind 127.0.0.1");
}
