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
}
