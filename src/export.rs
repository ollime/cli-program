// Handles exporting file to HTML
// use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io;
use std::io::Read;
use std::io::Write;

pub struct Export {

}

impl Export {
    pub fn export_as_html(input_value: &String, tab_name: String) -> io::Result<()> {
        let path: String = format!("data/{}.html", tab_name);
        let path = Path::new(&path);
        let display = path.display();

        // opens a file in write-only mode
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        // inserts input_value into the file
        match file.write_all(&input_value.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    
        // let mut file = match File::open(&path) {
        //     Err(why) => panic!("couldn't open {}: {}", display, why),
        //     Ok(file) => file,
        // };
    
        // let mut s = String::new();
        // match file.read_to_string(&mut s) {
        //     Err(why) => panic!("couldn't read {}: {}", display, why),
        //     Ok(_) => print!("{} contains:\n{}", display, s),
        // }

        Ok(())
    }
}

// struct Export {
//     data: HashMap<String, String>
// }

// impl Export {
//     pub fn new(data: HashMap<String, String>) -> Self {
//         Self {
//             data: data,
//         }
//     }
// }