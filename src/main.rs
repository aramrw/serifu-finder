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
fn prompt_path<E: std::io::Write>(
    p: &mut Promptuity<E>,
) -> Result<String, Error> {

    let mut path = p
        .prompt(Input::new("Enter Path to Subtitle Files").with_placeholder("~\\Desktop"))?
        .trim_matches('\"')
        .to_string();

    while !Path::new(&path).exists() || !Path::new(&path).is_dir() {
        if !Path::new(&path).exists() {
            eprintln!("`{}` does not exist.", path);
        } else {
            eprintln!("`{}` is not a directory.", path);
        }
        path = p
            .prompt(Input::new("Enter Path to Subtitle Files").with_placeholder("~\\Desktop"))?
            .trim_matches('\"')
            .to_string();
    }

    Ok(path)
}
