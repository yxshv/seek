use std::{ env, process::{ self, Command } };
use colored::*;
use tabled::Table;

fn main() {
    
    let args: Vec<String> = env::args().collect();   

    if args.len() == 1 {
        seek::help_msg();
        return;
    } else if args.len() == 2 {
        if args[1] == "help" {
            seek::help_msg();
        } else if args[1] == "--app" {
            println!("{}", Table::new(seek::apps_list()));      
        } else if args[1] == "--path" {
            println!("{}", Table::new(seek::paths_list()));
        } else {
            match seek::get_app(&args[1]) {
                Ok(v) => {
                    Command::new(v).output().expect(&"Failed to run the app".red());
                },
                Err(e) => {
                    println!("{}", e.red());
                    process::exit(1);
                }
            };
        }
    } if args.len() == 3 {
        if args[1] == "--app" {
            match seek::get_app(&args[2]) {
                Ok(v) => {
                    println!("{}", v);
                },
                Err(e) => {
                    println!("{}", e.red());
                    process::exit(1);
                }
            };
        } else if args[1] == "--path" {
            match seek::get_path(&args[2]) {
                Ok(v) => {
                    println!("{}", v);
                },
                Err(e) => {
                    println!("{}", e.red());
                    process::exit(1);
                }
            };
        } else {
            match seek::get_app(&args[1]) {
                Ok(v) => {
                    match seek::get_path(&args[2]) {
                        Ok(p) => {
                            Command::new(v).arg(p).output().expect(&"Failed to run the app".red());
                        },
                        Err(e) => {
                            println!("{}", e.red());
                        }
                    };
                },
                Err(e) => {
                    println!("{}", e.red());
                    process::exit(1);
                }
            };
        }
    } else if args.len() == 4 {
        if args[1] == "--app" {
            seek::change_app(&args[2], &args[3]);
        } else if args[1] == "--path" {
            seek::change_path(&args[2], &args[3]);
        } else if args[2] == "--custom" {
            match seek::get_app(&args[1]) {
                Ok(v) => {
                    Command::new(v).arg(&args[3]).output().expect(&"Failed to run the app".red());
                },
                Err(e) => {
                    println!("{}", e.red());
                    process::exit(1);
                }
            };
        } else {
            seek::help_msg();
        }
    }   

}