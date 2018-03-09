use std::env;
use std::fs;
use std::process;
use std::path::PathBuf;

fn collect_parents(path: std::path::PathBuf, paths: &mut Vec<PathBuf>) -> &Vec<PathBuf> {
    &paths.push(PathBuf::from(path));

    loop {
        let parent = PathBuf::from(paths.last().unwrap().parent().unwrap());
        &paths.push(parent);
        if paths.last().unwrap() == &PathBuf::from("/") {
            break;
        }
    }

    return paths;
}

fn has_dotgit(path: &std::path::PathBuf) -> bool {
    if path.join(".git").exists() {
        return true;
    }
    else {
        return false;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("You must provide a directory as an argument");
        process::exit(1);
    }

    let path_string = fs::canonicalize(&args[1]).unwrap();
    let path = PathBuf::from(path_string);

    if !path.exists() {
        println!("Invalid path {:?}", path);
        process::exit(1);
    }

    let mut paths: Vec<PathBuf> = Vec::new();
    collect_parents(path, &mut paths);

    for path in &paths {
        if has_dotgit(&path) {
            println!("{}", path.display());
            process::exit(0);
        }
    }

    process::exit(1);
}
