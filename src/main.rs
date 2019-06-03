use std::fs::File;
use std::env;

fn usage() {
   println!("rsgrep PATTERN FILENAME");
}

fn main() {
   let filename = match env::args().nth(2) {
       Some(filename) => filename,
       None => {
            usage();
            return;
       }
   };

   let file = match File::open(&filename) {
       Ok(file) => file,
       Err(e) => {
            println!("An error occoured while opening file {}, {}", filename, e);
            return;
       }
   };
}