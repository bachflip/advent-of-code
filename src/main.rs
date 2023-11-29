pub mod _2015;

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
    let input = read_input(path);
    _2015::_1::solve();

    solve!(year, day, problem);
    
}

fn read_input(path: PathBuf) -> String {
    let input: String = fs::read_to_string(path).unwrap();
    input
}

#[macro_export]
macro_rules! solve {
    ( $year:path, $day:path, $problem:ident ) => {
        {
            $year::$day::solve();
        }
    };
}
