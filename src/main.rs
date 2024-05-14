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
fn try_again() -> Result<bool, Error> {
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    println!("\n");
    p.with_intro("Try again?").begin()?;
    let confirm = p.prompt(Confirm::new("Would You Like To Try Again?").with_default(false))?;

    Ok(confirm)
}

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

fn ask_prompts<E: std::io::Write>(
    p: &mut Promptuity<E>,
    path: String,
) -> Result<PromptResult, Error> {

    p.term().clear()?;
    p.with_intro("Serifu Finder").begin()?;

    let mut final_paths_vec: Vec<String> = Vec::new();

    if let Ok(entries) = read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    if let Some(str_path) = entry.path().to_str() {
                        final_paths_vec.push(str_path.to_string());
                    }
                }
            }
        }
    }

    let options: Vec<MultiSelectOption<String>> = final_paths_vec
        .iter()
        .map(|path| MultiSelectOption::new(path.clone().rsplit_once('\\').unwrap().1, path.clone()))
        .collect();

    let selected_paths = p.prompt(
        MultiSelect::new("Select Which Folders to Search", options)
            .with_hint("Select w/ Space | Submit w/ Enter")
            .with_required(true),
    )?;

    let serifu = p
        .prompt(Input::new("Enter Serifu to Find").with_required(true))?
        .trim_matches('\"')
        .to_string();

    p.with_outro(format!(
        "Searching through {} folders for `{}`...",
        selected_paths.len(),
        serifu
    ));

    p.finish()?;

    Ok(PromptResult {
        serifu,
        paths: selected_paths,
    })
}


