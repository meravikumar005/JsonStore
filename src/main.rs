use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::{env, process, fs, io};

#[derive(Serialize, Deserialize, Debug)]
struct JsonStore {
    data:HashMap<String,String>
}

impl JsonStore {
     fn new() -> Self {
       Self {
        data:HashMap::new(),
       }
     }

    fn load_from_file(filename:&str) -> Result<Self, io::Error> {
        let content = fs::read_to_string(&filename).unwrap_or_else(|_| "{}".to_string());
        let store:Self = serde_json::from_str(&content).unwrap_or_else(|_| Self::new());
        Ok(store)
    }

    fn save_to_file(&self, filename:&str) -> Result<(), io::Error> {
        let content = serde_json::to_string_pretty(&self).expect("Error serializing data to json");
        fs::write(filename, content).expect("Error writing data to file");
        Ok(())
    }

    fn set(&mut self, key:&str, value:&str, filename:&str)-> Result<(), io::Error> {
        self.data.insert(key.to_string(), value.to_string());
        self.save_to_file(filename)?;
        println!("saved {}", key);
        Ok(())
    }

    fn get(&self, key:&str) -> Result<(), io::Error> {
        match self.data.get(key) {
            Some(value) => println!("Name: {}", value),
            None => println!("No key found")
        }

        Ok(())
    }

    fn delete(&mut self, key:&str, filename:&str) -> Result<(), io::Error> {
        if self.data.remove(key).is_some() {
            self.save_to_file(filename)?;
        } else {
            println!("No key found");
        }

        Ok(())
    }
}


fn main() {
    let filename = "storage.json";
    let mut store = JsonStore::load_from_file(filename);

    let args:Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: set <key> <value> | get <key> | delete <key>");
        process::exit(1);
    }

    match args[1].as_str() {
        "set" => {
            store.set(args[2].as_str(), args[3].as_str(), filename);
        }

        "get" => {
            store.get(args[2].as_str());
        }

        "delete" => {
            store.delete(args[2].as_str(), filename);
        }

        _ => {
            eprintln!("Invalid arguments");
            process::exit(1);
        }
    }

}
