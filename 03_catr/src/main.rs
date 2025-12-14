use catr::Config; // Need to import Config
use clap::Parser;   // Need to import the Parser trait to call ::parse()

fn main() {
    if let Err(e) = catr::run(Config::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

