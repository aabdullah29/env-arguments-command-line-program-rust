// console program 
// we pass two arguments when we call the program on console like:
// cargo run --hello world


const DATABASE_NAME :&str = "env.db";

fn main() {
    let mut args = std::env::args().skip(1);
    // let key = args.next().unwrap();
    let key = args.next().expect("keay was not found");
    let value = args.next().expect("value was not found");
    println!("key is: {} and value is: {}", key, value);

    // part 1:
    //just write one key and value without Database Struct 
    // let contents = format!("{}\t{}\n", key, value);
    // std::fs::write("env-argument.db", contents).unwrap();
    // let write_result = std::fs::write(DATABASE_NAME, contents);
    // match write_result {
    //     Ok(()) => println!("write successfully"),
    //     Err(e) => println!("write failed: {}", e),        
    // }


    // part 2:
    // save multipile keys and values
    let mut database = Database::new().expect("Database::new() failed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    database.flush().expect("flush failed");
}

struct Database {
    map: std::collections::HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map: std::collections::HashMap<String, String> = std::collections::HashMap::new();

        // raed from db file 
        // std::fs::read_to_string(DATABASE_NAME).expect("read from db file failed");
        // std::fs::read_to_string(DATABASE_NAME)?;
        let contents = match  std::fs::read_to_string(DATABASE_NAME) {
            Ok(c) => c,
            Err(_) => {
                // return Err(e)
                match std::fs::write(DATABASE_NAME, "") {
                    Ok(()) => {
                        println!("new db write successfully");
                        "".to_string()
                },
                    Err(e) => return Err(e),        
                }
            },
        };
        
        // parse string contents and insert in map
        for line in contents.lines() {
            // let pair = line.split_once('\t').expect("database crashes");
            // let (key, value) = line.split_once('\t').expect("database crashes");

            let mut pair = line.splitn(2, '\t');
            let key = pair.next().unwrap();
            let value = pair.next().unwrap();
            map.insert(key.to_string(), value.to_owned());
        }
        // return new Database instance
        Ok(Database { 
            map,
         })
    }

    // insert in map
    fn insert(&mut self, key: String, value: String) {
        // self.map.insert(key, value);
        match self.map.insert(key, value){
            Some(s) => println!("valu was already exist: {}", s),
            None => println!("insert successfully"),
        }
    }

    // reset and update db file
    // we can use it in drope
    fn flush (self) -> std::io::Result<()> {
        let mut contents = String::new();
        // for pairs in self.map{
        //     let db_pair = format!("{}\t{}\n", pairs.0, pairs.1);
        //     // contents = contents + &db_pair;pairs
        //     contents.push_str(&db_pair);
        // }
        for (key, value) in self.map{
            let db_pair = format!("{}\t{}\n", key, value);
            // contents = contents + &db_pair;
            contents.push_str(&db_pair);
        }

        // todo!("flush")
        std::fs::write(DATABASE_NAME, contents)
    }
}




// drop will run automatically at the end ?
// impl Drop for Database {
//     fn drop(&mut self) {
//         let mut contents = String::new();
//         for (key, value) in &self.map{
//             let db_pair = format!("{}\t{}\n", key, value);
//             contents.push_str(&db_pair);
//         }
//         let _= std::fs::write(DATABASE_NAME, contents);
//         println!("drop is called.")
//     }
// }