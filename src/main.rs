use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::fs;
use std::process;
use std::path::PathBuf;

struct Config {
    targetpath: String,
    extensions: Vec<String>,
    quiet: bool
}

#[derive(PartialEq)]
enum Keyword {
    None,
    Extension,
    Quiet
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Config, String> {
        if args.len() < 2 {
            return Err("No arguments".to_string());
        }
        let targetpath = args[1].clone();
        let mut extensions: Vec<String> = vec![];

        let mut n = 2;
        let mut cur_option = Keyword::None;
        let mut quiet = false;

        while n < args.len() {
            let cur_arg = &args[n].clone().to_owned()[..];
            match cur_arg {
                "-e" => cur_option = Keyword::Extension,
                "-q" => {
                    cur_option = Keyword::Quiet;
                    quiet = true;
                },
                _ => {
                    if cur_option == Keyword::None {
                        return Err("Argument Parsing Error".to_string());
                    }
                    match cur_option {
                        Keyword::Extension => {extensions.push(args[n].clone())},
                        _ => {return Err("Argument Parsing Error".to_string());}
                    }
                }
            }
            n += 1;
        }
        Ok(Config { targetpath: targetpath, extensions: extensions, quiet: quiet })
    }
}

struct  Counter {
    line_count: HashMap<String, u32>,
    char_count: HashMap<String, u32>
}

fn main() {
    let args = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|_err| {
        println!("Argument Parsing Error");
        process::exit(1);
    });

    let targetdir = PathBuf::from(&config.targetpath);
    let res = fs::canonicalize(targetdir);
    if res.is_err() {
        println!("Undefined Path: {}", config.targetpath);
    }
    let path = res.unwrap();
    let mut counter = Counter {
        line_count: HashMap::new(),
        char_count: HashMap::new()
    };
    for extension in &config.extensions {
        counter.line_count.insert(extension.clone(), 0);
        counter.char_count.insert(extension.clone(), 0);
    }
    let (line_count, char_count) = read_dir(path, &mut counter, &config);
    println!("\nline count: {}, char count: {}\n", line_count, char_count);
    for (lk, lv) in counter.line_count {
        println!("{} \t line: {}, char: {}", lk, lv, counter.char_count.get(&lk).unwrap());
    }
}

fn read_dir(path: PathBuf, counter: &mut Counter, config: &Config) -> (u32, u32) {
    if !config.quiet{println!("Entry directory: {}", path.to_string_lossy());}
    let paths = fs::read_dir(path).unwrap();
    let mut line_count = 0;
    let mut char_count = 0;

    for path in paths {
        if path.is_err() {
            println!("Read Path Error: {:?}", path.err());
            continue;
        }
        let path = path.unwrap().path();
        if path.is_dir() {
            let (lc, cc) = read_dir(path, counter, config);
            line_count += lc;
            char_count += cc;
        } else if path.is_file() {
            if path.extension().is_none() {
                continue;
            }
            let extension = String::from(path.extension().unwrap().to_string_lossy());
            if !config.extensions.contains(&extension) { continue;}
            if !config.quiet{println!("Parsing File   : {}", path.to_string_lossy());}
            let (lc, cc) = read_file(path);
            line_count += lc;
            char_count += cc;
            counter.line_count.insert(extension.clone(), *counter.line_count.get(&extension).unwrap() + lc);
            counter.char_count.insert(extension.clone(), *counter.char_count.get(&extension).unwrap() + cc);
        } else {
            println!("Undefined Read Type: {}", path.to_string_lossy());
        }
    }
    return (line_count, char_count);
}

fn read_file(path: PathBuf) -> (u32, u32) {
    let file = File::open(&path);
    if file.is_err() {
        println!("File Open Error: {}", path.to_string_lossy());
        return (0, 0);
    }
    let reader = BufReader::new(file.unwrap());
    let mut line_count: u32 = 0;
    let mut char_count: u32 = 0;
    for line in reader.lines() {
        if line.is_err() {continue;}
        let cc = line.unwrap().trim().len() as u32;
        if cc == 0 {continue;}
        line_count += 1;
        char_count += cc;
    }
    return (line_count, char_count);
}