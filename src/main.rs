
use std::env;
pub mod lib;
use std::fs;
use std::process::Command;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let path = lib::check_args(); // gets cl args
    println!("setting GRUB wallpaper as: {}", path);

    fs::copy(path, "./image.png");
    Command::new("mv").arg("./image.png").arg("/etc/default/background.png").spawn().expect("command failed"); // moves into grub folder

    let mut found = 0;
    let mut line_num = 0;
    if let Ok(lines) = lib::read_lines("/etc/default/grub") {
        'x: for line in lines {
            if let Ok(text) = line {
                if text.contains("GRUB_BACKGROUND="){
                    found = 1;
                    println!("working");
                    break 'x;
                }
                line_num += 1;
            }
        }
    } else {
        panic!("you do not have a valid grub config at /etc/default/");
    }
    if found == 0 {
        let mut file = OpenOptions::new().append(true).open("/etc/default/grub").expect("failed to open file");
        file.write_all("GRUB_BACKGROUND=\"/etc/default/background.png\"".as_bytes()).expect("failed to write to file");
    }
}
