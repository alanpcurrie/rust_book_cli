use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = rust_book_cli::Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("searching for {}", config.query);
    println!("in file {}", config.file_path);

    if let Err(e) = rust_book_cli::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();
//     Config { query, file_path }
// }

// idiomatic refactor to remove clone
// fn parse_config(args: &[String]) -> Config {
//     let query = args
//         .get(1)
//         .map(|s| s.to_owned())
//         .unwrap_or_else(String::new);
//     let file_path = args
//         .get(2)
//         .map(|s| s.to_owned())
//         .unwrap_or_else(String::new);

//     Config { query, file_path }
// }
