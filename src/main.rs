extern crate getopts;

use getopts::Options;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] FILE", program);
    print!("{}", opts.usage(&brief));
}

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
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("v", "verbose", "turn on verbose logging");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let verbose = matches.opt_present("v");
    let path_arg = args.last().unwrap();

    // let input = if !matches.free.is_empty() {
    //     matches.free[0].clone()
    // } else {
    //     print_usage(&program, opts);
    //     return;
    // };
    //do_work(&input, output);

    if verbose {
        println!("args are {:?}", &args);
        println!("path_arg is {:?}", &path_arg);
    }

    let path_string = fs::canonicalize(path_arg).unwrap_or("/");
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
