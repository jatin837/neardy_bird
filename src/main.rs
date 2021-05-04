use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main() {
    let screen_path = Path::new("./screen.txt");
    let display = screen_path.display();

    let mut file = match File::open(&screen_path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
    s[10] = "a";
}
