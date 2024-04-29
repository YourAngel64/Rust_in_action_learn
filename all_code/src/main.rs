#![allow(dead_code)]

use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]

enum FileState{
    Open,
    Closed,
}

#[derive(Debug)]
struct File{
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match *self{
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File{
    fn new(name: &str) -> File{
        File{
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }
}

fn main(){
    let mut f1 = File::new("f1.txt");
    f1.data = vec![100, 12, 45, 64];

    println!("{:?}", f1);
    println!("{}", f1);
}