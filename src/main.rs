use std::fs;
use std::path::Path;
use std::env;
use bytesize::ByteSize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use colored::*;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("{}", format!("Usage: {} <directory>", args[0]).red());
        return;
    }

    let path = &args[1];
    let metadata = fs::metadata(path).expect("Unable to read metadata");
    if !metadata.is_dir() {
        eprintln!("{}", "Provided path is not a directory".red());
        return;
    }

    let start_time = Instant::now();

    let mut file_count = 0;
    let mut dir_count = 0;
    let mut total_size = 0;

    let mp = MultiProgress::new();
    let pb = mp.add(ProgressBar::new_spinner());
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg}")
        .expect("Failed to set template")
        .tick_strings(&["⠋", "⠙", "⠴", "⠦", "⠇", "⠸", "⠼", "⠴", "⠦", "⠇", "⠸"]));

    pb.set_message("Processing...");
    visit_dirs(Path::new(path), &mut file_count, &mut dir_count, &mut total_size).expect("Error reading directory");
    pb.finish_with_message("Done!");

    let duration = start_time.elapsed();

    println!("{}", format!("Total size: {}", ByteSize(total_size)).cyan());
    println!("{}", format!("File count: {}", file_count).cyan());
    println!("{}", format!("Directory count: {}", dir_count).cyan());
    println!("{}", format!("Execution time: {:.2?}", duration).cyan());
}

fn visit_dirs(dir: &Path, file_count: &mut usize, dir_count: &mut usize, total_size: &mut u64) -> std::io::Result<()> {
    if dir.is_dir() {
        *dir_count += 1;
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, file_count, dir_count, total_size)?;
            } else {
                *file_count += 1;
                if let Ok(metadata) = fs::metadata(&path) {
                    *total_size += metadata.len();
                }
            }
        }
    }
    Ok(())
}
