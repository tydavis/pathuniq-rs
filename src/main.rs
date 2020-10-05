use std::env;

fn main() {
    let key = "PATH";
    match env::var_os(key) {
        Some(path) => {
            let mut paths = env::split_paths(&path).enumerate().collect::<Vec<_>>();
            paths.sort_by(|(_, a), (_, b)| a.cmp(b));
            paths.dedup_by(|(_, a), (_, b)| a.eq(&b));
            paths.sort_by(|(a, _), (b, _)| a.cmp(b));
            let k = env::join_paths(paths.iter().map(|(_, x)| x))
                .unwrap_or_else(|error| panic!("error joining path: {:?}", error));
            let newpath = k
                .into_string()
                .unwrap_or_else(|error| panic!("could not translate path into str: {:?}", error));
            println!("export PATH={}", newpath);
        }
        None => println!("variable {} is not defined in the environment.", key),
    }
}
