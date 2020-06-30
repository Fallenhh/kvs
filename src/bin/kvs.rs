use clap::{App};
use std::process::exit;

fn main() {
    let matches = App::new("kvs")
                  .version(env!("CARGO_PKG_VERSION"))
                  .author(env!("CARGO_PKG_AUTHORS"))
                  .about("A simple key-value database")
                  .subcommand(App::new("set")
                    .arg("<KEY>")
                    .arg("<VALUE>"))
                  .subcommand(App::new("get")
                    .arg("<KEY>"))
                  .subcommand(App::new("rm")
                    .arg("<KEY>"))
                  .get_matches();

    match matches.subcommand() {
        ("set", Some(_matches)) => {
            eprint!("unimplemented");
            exit(1);
        }
        ("get", Some(_matches)) => {
            eprint!("unimplemented");
            exit(1);
        }
        ("rm", Some(_matches)) => {
            eprint!("unimplemented");
            exit(1);
        }
        _ => panic!()
    }
}
