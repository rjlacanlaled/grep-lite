use std::{env, fs::File, io::BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let search_term = &args[1];
    let path = &args[2];
    let file = File::open(path).expect("Could not open file");
    let reader = std::io::BufReader::new(file);

    for line in reader.lines(){
        if line.is_err() {
            continue;
        }
        let content = line.unwrap();
        if content.contains(search_term) {
            println!("{}", content);
        }
    }
}
