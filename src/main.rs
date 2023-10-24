use std::{fs::File, io::BufRead};
use regex::Regex;
use clap::{App,Arg};


fn main() {
    let args = App::new("grep-lite")    
    .version("0.1")
    .about("searches for patterns")
    .args(&[
        Arg::with_name("pattern")
        .help("The pattern to search for")
        .takes_value(true)
        .required(true), 
      Arg::with_name("path")
        .help("The file path to search")
        .takes_value(true)
        .required(true)])
    .get_matches();

    let search_term = args.value_of("pattern").unwrap();
    let re = Regex::new(search_term).unwrap();
    let path = args.value_of("path").unwrap();
    let file = File::open(path).expect("Could not open file");
    let reader = std::io::BufReader::new(file);

    for line in reader.lines(){
        if line.is_err() {
            continue;
        }
        let content = line.unwrap();
        if  re.find(&content).is_some() {
            println!("{}", content);
        }
    }
}
