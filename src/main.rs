use std::{path::PathBuf, env, fs};

fn main() {
    let project_dir = env!("CARGO_MANIFEST_DIR");
    let mut path: PathBuf = [project_dir, "input"].iter().collect();

    let args = env::args().collect::<Vec<String>>();
    if args.len() < 4 {
        panic!("cmd year day problem");
    }
    let year = &args[1];
    let day = &args[2];
    let problem = &args[3];
    path.push(year);
    path.push(day);
    path.push(problem);

    println!("path {}", read_input(path));
}

fn read_input(path: PathBuf) -> String {
    let input: String = fs::read_to_string(path).unwrap();
    input
}
