use std::fs::File;
use std::io::{BufRead, BufReader};
use termion::color;

use crate::{get_input, print_error, print_success};
use crate::write::write_file;

fn print_parameters(params: &Vec<&str>) {
    let mut counter = 0;
    for p in params.iter() {
        println!(
            "{}{}:{} {}",
            color::Fg(color::Red),
            counter,
            color::Fg(color::Reset),
            p
        );
        counter += 1;
    }
}

fn ask_user(params: &Vec<&str>) -> Option<Vec<Option<usize>>> {
    print_parameters(params);
    let i = get_input("Which parameters do you want to delete? ");
    let max = params.len() - 1;
    let split: Vec<&str> = i.split(' ').collect();
    let mut error = false;
    if split.len() > 0 {
        let nums: Vec<Option<usize>> = split
            .into_iter()
            .map(|e| {
                if let Ok(e) = e.parse() {
                    if e > max {
                        error = true;
                        None
                    } else {
                        Some(e)
                    }
                } else {
                    error = true;
                    None
                }
            })
            .collect();
        if error {
            None
        } else {
            Some(nums)
        }
    } else {
        None
    }
}

pub fn start(br: BufReader<File>) {
    let mut lines: Vec<String> = Vec::new();
    let mut error = true;
    for eresult in br.lines().into_iter() {
        let e = eresult.unwrap();
        let split: Vec<&str> = e.split('=').collect();
        let line;
        if split.len() > 1 {
            if split[0] == "GRUB_CMDLINE_LINUX_DEFAULT" {
                error = false;
                let mut right_side = split[1..].join("=").to_string();
                let r_len = right_side.len() - 1;
                right_side.drain(0..1);
                right_side.drain(r_len - 1..r_len);
                let mut params: Vec<&str> = right_side.split(' ').collect();
                if let Some(v) = ask_user(&params) {
                    let mut counter = 0;
                    for n in v.into_iter() {
                        if let Some(n) = n {
                            if n as i8 - counter > -1 {
                                params.remove(n - counter as usize);
                                counter += 1;
                            } else {
                                error = true;
                            }
                        } else {
                            error = true;
                        }
                    }
                } else {
                    error = true;
                }
                line = format!(
                    "GRUB_CMDLINE_LINUX_DEFAULT={}{}{}",
                    '"',
                    params.join(" ").as_str(),
                    '"'
                );
            } else {
                line = split.join("=");
            }
        } else {
            line = e;
        }
        lines.push(line);
    }
    write_file(lines);
    if error {
        print_error("Error removing grub parameters");
    } else {
        print_success("Successfully removed grub parameters");
    }
}
