use promptuity::prompts::{Confirm, Input, MultiSelect, MultiSelectOption};
use promptuity::themes::FancyTheme;
use promptuity::{Error, Promptuity, Term};
use std::fs::read_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

struct PromptResult {
    #[allow(dead_code)]
    serifu: String,
    #[allow(dead_code)]
    paths: Vec<String>,
}

#[derive(Debug)]
struct MatchResult {
    #[allow(dead_code)]
    path: String,
    #[allow(dead_code)]
    serifu: String,
    #[allow(dead_code)]
    line: u32,
}

fn main() {
    println!("Hello, world!");
}
