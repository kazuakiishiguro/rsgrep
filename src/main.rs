extern crate regex;
use regex::Regex;
use std::io::{BufReader, BufRead};
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

   let pattern = match env::args().nth(1) {
       Some(pattern) => pattern,
       None => {
            usage();
            return;
       }
   };

   let reg = match Regex::new(&pattern) {
       Ok(reg) => reg,
       Err(e) => {
            println!("invalid regrep {} : {}",pattern, e);
            return;
       }
   };

   let input = BufReader::new(file);
   for line in input.lines() {
       let line = match line {
           Ok(line) => line,
           Err(e) => {
               println!("An error occoured while reading a line {}", e);
               return;
           }
       };

       if reg.is_match(&line) {
          println!("{}", line);
       }
   }
 }