use std::collections::HashSet;
use std::env;

fn main() {
    let key = "PATH";
    match env::var_os(key) {
        Some(path) => {
            let mut paths = env::split_paths(&path).collect::<Vec<_>>();
            let mut uniques = HashSet::new();
            paths.retain(|e| uniques.insert(e.clone()));
            let k = env::join_paths(paths).unwrap();
            let newpath = k.into_string().unwrap();
            println!("export PATH={}", newpath);
        }
        None => println!("{} is not defined in the environment.", key),
    }
}
