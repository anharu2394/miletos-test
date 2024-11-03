use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use serde_json::{Map, Value};

#[derive(Debug)]
pub struct Config {
    pub data: Map<String, Value>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            data: Map::new(),
        }
    }

    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line?.trim().to_string();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = parse_line(&line) {
                insert_key_value(&mut self.data, key, value);
            }
        }
        Ok(())
    }
}

fn parse_line(line: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = line.splitn(2, '=').map(|s| s.trim()).collect();
    if parts.len() == 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None
    }
}

fn insert_key_value(data: &mut Map<String, Value>, key: String, value: String) {
    let mut keys: Vec<&str> = key.split('.').collect();
    let last_key = keys.pop().unwrap();

    let mut current = data;
    for &k in &keys {
        current = current
            .entry(k.to_string())
            .or_insert_with(|| Value::Object(Map::new()))
            .as_object_mut()
            .unwrap();
    }
    current.insert(last_key.to_string(), Value::String(value));
}

fn main() {
    let mut config = Config::new();

    if let Err(e) = config.load_from_file("example/config.conf") {
        eprintln!("Error loading config: {}", e);
    } else {
        println!("{:#?}", config.data);
    }
}
