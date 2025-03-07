use std::env;
use std::fs;

fn main() {
   let args: Vec<String> = env::args().collect();
//    let query = &args[1];
//    let file_path = &args[2];
    let config = parse_config(&args);
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

   let contents = fs::read_to_string(config.file_path)
      .expect("Something went wrong reading the file");
      println!("With text:\n{contents}");
}



struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
   let query = &args[1].clone();
   let file_path = &args[2].clone();
   
   Config { query: query.to_string(),file_path: file_path.to_string() }

}