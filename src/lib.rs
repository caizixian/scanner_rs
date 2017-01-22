use std::io::stdin;
use std::collections::LinkedList;

pub struct Scanner<> {
    tokens: LinkedList<String>
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner { tokens: LinkedList::new() }
    }
    pub fn next_line(&self) -> String {
        let mut temp = String::new();
        stdin().read_line(&mut temp).unwrap();
        temp
    }
    pub fn next<T>(&mut self) -> T
        where T: std::str::FromStr,
              T::Err: std::fmt::Debug {
        while self.tokens.is_empty() {
            let mut temp = String::new();
            stdin().read_line(&mut temp).unwrap();
            for token in temp.split_whitespace() {
                self.tokens.push_back(token.to_string());
            }
        }
        self.tokens.pop_front().unwrap().parse().unwrap()
    }
}