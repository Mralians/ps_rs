use process::process;
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let pid = args.get(1).unwrap().trim().parse::<u32>().unwrap();
    let state = process::get_process_info(pid).unwrap();
    println!("{state:#?}");
}
