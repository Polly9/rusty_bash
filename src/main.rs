//SPDX-FileCopyrightText: 2022 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use std::io;
use std::io::Write;

mod parser;
mod elements;

//use elements::Element;

fn prompt() {
    print!("$ ");
    io::stdout()
        .flush()
        .unwrap();
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    line
}

fn main() {
    loop {
        prompt();
        let line = read_line();
        match parser::top_level_element(line) {
            Some(ans) => ans.exec(),
            None => panic!("")
        }
    }
}


#[test]
fn parse() -> () {
    let ans = parser::top_level_element("echo hoge".to_string());
    assert_eq!(ans.1[0], CString::new("echo").unwrap());
    assert_eq!(ans.1[1], CString::new("hoge").unwrap());
}
