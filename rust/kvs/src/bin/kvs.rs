#[macro_use]
extern crate clap;

use clap::App;
use kvs::KvStore;

fn main() {
    let mut kv_store = KvStore::new();

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    if matches.is_present("V") {
        println!("version: {}", "1.0");
    } else {
        match matches.subcommand() {
            ("set", Some(matches)) => {
                let k = matches.value_of("KEY").unwrap();
                let v = matches.value_of("VALUE").unwrap();
                kv_store.set(k.to_string(), v.to_string());
            },
            ("get", Some(matches)) => {
                let k = matches.value_of("KEY").unwrap();
                match kv_store.get(k.to_string()) {
                    Some(v) => println!("value of {} is {}", k, v),
                    None => println!("found no value of key {}", k)
                }
            },
            ("rm", Some(matches)) => {
                let k = matches.value_of("KEY").unwrap();
                kv_store.remove(k.to_string());
            },
            _ => {
                eprintln!("invalid args");
                std::process::exit(1);
            }
        }
    }
}