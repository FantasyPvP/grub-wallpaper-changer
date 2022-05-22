
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

pub fn check_args() -> String {
    let mut args: Vec<String> = env::args().collect();

    let mut path = String::new();
    match args.get_mut(1) {  // panics if no args are given
        Some(x) => { path = args[1].clone() },
        None => panic!("no command line argument detected,\nplease enter path of image to set as wallpaper")
    }
    match args.get_mut(2) {  // panics if more than one arg is given
        None => (),
        Some(x) => panic!("multiple command line arguments detected! please enter only 1 file path")
    }
    path // returns the path to the file
}
