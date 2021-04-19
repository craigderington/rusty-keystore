use std::collections::HashMap;

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        let contents: String = std::fs::read_to_string("kvs.db")?;

        for line in contents.lines() {
            let mut chunks = line.splitn(2, "\t");
            let key = chunks.next().expect("No key");
            let value = chunks.next().expect("No value");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database {
            map: map
        })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(&self) -> std::io::Result<()> {
        let mut contents = String::new();
        for (key, value) in &self.map {
            let kvpair = format!("{}\t{}\n", key, value);
            contents.push_str(&kvpair);
        }
        std::fs::write("kvs.db", contents)
    }
}


fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("No key was entered.");
    let value = arguments.next().unwrap();
    // write the key value pair to the datafile
    println!("Key: {}, Value: {}", key, value);
    // let contents = format!("{}\t{}\n", key, value);
    // std::fs::write("kvs.db", contents).unwrap();
    // call new on Database
    let mut database = Database::new().expect("Creating database failed...");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    database.flush().unwrap();
}
