use std::env;
use std::fs;
// cargo run -- searchstring example-filename.txt
// output args = [
//  "target/debug/rust_book_cli",
// "searchstring",
//  "example-filename.txt",
// ]
fn main() {
    let msg = "Should have been able to read the file";
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    let contents = fs::read_to_string(file_path).expect(msg);
    // Prints and returns the value of a given expression for quick and dirty debugging.
    dbg!(query, file_path);
    println!("searching for {}", query);
    println!("in file {}", file_path);
    println!("With text:\n{contents}");
}
