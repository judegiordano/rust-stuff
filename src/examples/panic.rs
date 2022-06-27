use std::fs;
use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::net::IpAddr;

pub fn example() {
    let done = read_username_shorthand();
    println!("{:#?}", done);
    let username = read_username_from_file();
    println!("{:#?}", username);
}

#[allow(dead_code)]
pub fn read() {
    // panic!("crash and burn");
    let file = File::open("hello.txt");
    let done = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    println!("{:#?}", done);
}

#[allow(dead_code)]
fn unwrap_closure() {
    File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

pub fn read_username_from_file() -> Result<String, io::Error> {
    let file = File::open("username.txt");
    let mut file = match file {
        Ok(data) => data,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn read_username_shorthand() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

pub fn read_username_super_shorthand() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

pub fn read_username_super_duper_shorthand() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}

pub fn unwrap() -> IpAddr {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    home
}
