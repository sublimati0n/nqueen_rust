use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // 引数の数が足りない
            return Err("not enough arguments");
        }
        let query: String = args[1].clone();
        let filename: String = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f: File = File::open(config.filename).expect("file not found");

    let mut contents: String = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{contents}");

    Ok(())
}

pub struct TimeKeeper {
    pub start_time: Instant,
    pub time_threshold_seconds: u64,
}

impl TimeKeeper {
    pub fn is_time_over(&self) -> bool {
        let diff = self.start_time.elapsed();
        diff.as_secs() >= self.time_threshold_seconds
    }
}
