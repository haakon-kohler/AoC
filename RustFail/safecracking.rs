//use std::fs::File;
//use std::io::{self, BufRead};
//use std::path::Path;

use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {

    print!(read_lines("instructions.txt"))

    //The instructions file instructions.txt must exist in current path
    //if let Ok(lines) = read_lines("instructions.txt") {
        //for line in lines.map_while(Result::ok) {
            //println!("{}", line);
        //}
    //}
}

// Returns an Iterator struct; want to know what that is!
//fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
//where P: AsRef<Path>, {
//    let file = File::open(filename)?;
//    Ok(io::BufReader::new(file.lines()))
//}
