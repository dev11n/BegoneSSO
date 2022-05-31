#![allow(non_snake_case)]

use std::{
    env, fs,
    io::{stdin, Read},
    path::Path,
};

/// Startup function.
fn main() {
    println!("Made by devin (938880040169525358) - https://github.com/dev11n/BegoneSSO");
    println!("Fuck GameGuard, Fuck SSO White-Knights, Fuck Clara Plasticland");
    remove_gg(env::args().collect());
}

/// Executes the program with the specified arguments.
fn remove_gg(args: Vec<String>) {
    let len = args.len();
    if len < 2 {
        println!("Invalid amount of arguments passed! Fed {len} while expecting 2.");
        println!("USAGE: BegoneSSO.exe [path_to_sso]");
        return;
    }
    let path = &args[1];
    println!("Checking path, please wait...");
    if check_path(path) {
        println!("Path exists, please uninstall Star Stable manually via the uninstaller and then press ENTER when you are done.");
        pause();
        println!("----# CONFIRMATION #----");
        println!("Dirs to remove: {path}");
        println!("Files to remove: C:\\Windows\\System32\\GameMon.des");
        println!("Does this look right? If not, close the program!");
        pause();
        pause();
        println!("----# CONFIRMED #----");
        remove_path(path);
        remove_file("C:\\Windows\\System32\\GameMon.des");
        println!("Finished!");
    } else {
        println!("Failed to find the path, are you sure its correct?");
    }
}

/// Checks if the specified path exsists **AND** is a directory.
fn check_path(path: &String) -> bool {
    Path::new(path).is_dir()
}

/// Removes an entire directory, regardless of whether its empty or not.
fn remove_path(path: &String) {
    if check_path(path) {
        {
            let this = fs::remove_dir_all(path);
            match this {
                Ok(t) => t,
                Err(e) => panic!("Failed unwrapping in remove_path(), Error: {}", e),
            }
        }
    };
}

/// Removes a single file.
fn remove_file(path: &str) {
    if Path::new(path).is_file() {
        {
            let this = fs::remove_file(path);
            match this {
                Ok(t) => t,
                Err(e) => panic!("Failed unwrapping in remove_file(), Error: {}", e),
            }
        }
    }
}

/// Pauses the program until ENTER has been pressed.
fn pause() {
    {
        let this = Read::read(&mut stdin(), &mut [0]);
        match this {
            Ok(t) => t,
            Err(e) => panic!("Failed unwrapping in pause(), Error: {}", e),
        }
    };
}
