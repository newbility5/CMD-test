use getopts::Options;
use std::env;

mod config;
mod db;
mod router;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("c", "config", "set config file path", "CONFIG");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(matches) => matches,
        Err(_) => {
            print_help(&program, opts);
            return;
        }
    };
    if matches.opt_present("h") || !matches.free.is_empty() {
        print_help(&program, opts);
        return;
    };
    let config_file = matches.opt_str("c");
    if let Some(config_file) = config_file {
        let config = config::Config::from(config_file);
        if let Ok(config) = config {
            router::init(config).await.unwrap();
        } else {
            print_help(&program, opts);
            return;
        }
    } else {
        print_help(&program, opts);
        return;
    }
}

fn print_help(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}
