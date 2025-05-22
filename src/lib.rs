use serde::Deserialize;
use std::fs::{self, File};
use std::io::{self, Write};

#[derive(Deserialize)]
pub struct User {
    pub user: String,
    pub pass: String,
    pub count: usize,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.user, self.pass, self.count)
    }
}

// This repo is an exercise in rolling out auth, so the db is simply a .txt file
// However we may replace these implementations to with any database we desire.
const FILE_NAME: &str = "db.txt";

pub fn write_string_to_file(contents: &str) -> io::Result<()> {
    let mut curr: String = read_file_to_string();
    curr.push_str(contents);
    curr.push_str("\n");

    let mut file = File::create(FILE_NAME)?;
    file.write_all(curr.as_bytes())?;
    Ok(())
}

pub fn read_file_to_string() -> String {
    let output: String = fs::read_to_string(FILE_NAME).unwrap();
    output
}

pub fn where_row_match(target_str: &str) -> bool {
    let to_search = read_file_to_string();
    for row_str in to_search.split("\n") {
        if target_str == row_str {
            return true;
        }
    }
    return false;
}

pub fn increment_user(target_user: &str) -> () {
    let mut curr: String = String::new();

    let to_search = read_file_to_string();
    for row_str in to_search.split("\n") {
        if row_str.is_empty() {
            continue;
        }
        let row_user = row_str.split_whitespace().next().unwrap();

        if row_user == target_user {
            let row_count: usize = row_str
                .split_whitespace()
                .nth(2)
                .unwrap()
                .parse::<usize>()
                .unwrap()
                + 1;
            let new_row: String = row_str
                .split_whitespace()
                .take(2)
                .collect::<Vec<&str>>()
                .join(" ");
            curr = format!("{curr}{new_row} {row_count}\n");
        } else {
            curr = format!("{curr}{row_str}\n");
        }
    }

    let mut file = File::create(FILE_NAME).unwrap();
    file.write_all(curr.as_bytes()).unwrap();
    ()
}
