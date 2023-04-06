use process;
use std::fs::File;
use std::io::Read;
use std::process::id;
fn main() {
    let state = process::get_process_info(684).unwrap();
    println!("{state:#?}");
}
