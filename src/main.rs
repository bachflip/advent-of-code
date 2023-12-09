pub mod _1;
pub mod _2;
pub mod _3;
pub mod _4;
pub mod _5;
pub mod _6;
pub mod _7;
pub mod _8;
pub mod _9;

use std::{collections::BTreeMap, env, fs, path::PathBuf};

fn main() {
    let project_dir = env!("CARGO_MANIFEST_DIR");
    let mut path: PathBuf = [project_dir, "input"].iter().collect();

    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        panic!("cmd day problem");
    }

    let day = &args[1];
    let problem = &args[2];
    path.push(day);
    let input = read_input(path);

    let solver_key = format!("{}::_{}::_{}", env!("CARGO_CRATE_NAME"), day, problem);

    let mut solvers: BTreeMap<String, fn(String)> = BTreeMap::new();

    macro_rules! register {
        ( $solver:path ) => {{
            solvers.insert(fn_name($solver).to_string(), $solver);
        }};
    }

    register!(_1::_1);
    register!(_1::_2);
    register!(_2::_1);
    register!(_2::_2);
    register!(_3::_1);
    register!(_3::_2);
    register!(_4::_1);
    register!(_4::_2);
    register!(_5::_1);
    register!(_5::_2);
    register!(_6::_1);
    register!(_6::_2);
    register!(_7::_1);
    register!(_7::_2);
    register!(_8::_1);
    register!(_8::_2);
    register!(_9::_1);
    register!(_9::_2);

    solvers.get(&solver_key).unwrap()(input);
}

fn read_input(path: PathBuf) -> String {
    let input: String = fs::read_to_string(path).unwrap();
    input
}

fn fn_name<F>(_: F) -> &'static str
where
    F: Fn(String),
{
    std::any::type_name::<F>()
}
