use clap::{App};
use std::process::exit;
use std::path::PathBuf;
use kvs::KvStore;
use anyhow::Result;

fn main() -> Result<()>{
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

    let mut store = KvStore::open(".")?;
    match matches.subcommand() {
        ("set", Some(matches)) => {
            let key = matches.value_of("KEY").unwrap().to_owned();
            let value = matches.value_of("VALUE").unwrap().to_owned();
            store.set(key, value)?;
        }
        ("get", Some(matches)) => {
            let key = matches.value_of("KEY").unwrap().to_owned();
            let value = store.get(key)?;
            match value {
              Some(v) => println!("{}", v),
              None => println!("Key not found"),
            }

        }
        ("rm", Some(matches)) => {
            let key = matches.value_of("KEY").unwrap().to_owned();
            store.remove(key)?;
        }
        _ => panic!()
    }
    Ok(())
}
