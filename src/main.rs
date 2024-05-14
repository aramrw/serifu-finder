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
fn find_serifu(res: PromptResult) -> Result<Vec<MatchResult>, Error> {
    let valid_sub_exts: Vec<&str> = vec!["srt", "ass", "ssa", "vtt", "stl", "scc", "ttml", "sbv"];

    let mut valid_sub_file_paths: Vec<String> = Vec::new();

    // validate subtitle files before reading them
    for dir in res.paths {
        let entries = read_dir(dir)?.flatten();
        for file in entries {
            if let Some(ext) = file.path().extension() {
                let ext_str = ext.to_str().unwrap_or("");
                if valid_sub_exts.contains(&ext_str) {
                    if let Some(path) = file.path().to_str() {
                        valid_sub_file_paths.push(path.to_string());
                    }
                }
            }
        }
    }

    let mut matches: Vec<MatchResult> = Vec::new();

    for path in valid_sub_file_paths {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        reader.lines().enumerate().for_each(|(i, line)| {
            let line = line.as_ref().unwrap();
            // Replace this with your condition
            if line.contains(&res.serifu) {
                matches.push(MatchResult {
                    path: path.clone(),
                    serifu: line.to_string(),
                    line: i as u32,
                })
            }
        });
    }

    Ok(matches)
}

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


