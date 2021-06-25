use std::{collections::HashMap};

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Key was not found!");
    let value = arguments.next().unwrap();
    println!("The key is '{}' and the value is '{}'", key, value);
    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents).unwrap();

    let database = Database::new().expect("Creating Db failed");
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map= HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let mut pair = line.splitn(2, "\t");
            let key = pair.next().expect("Expected a key but got nothing!");
            let value = pair.next().expect("Expected a value but got nothing!");

            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database {
            map,
        })
    }
}