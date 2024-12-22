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

    let mp = MultiProgress::new();
    let spinner = mp.add(ProgressBar::new_spinner());
    spinner.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg}")
        .expect("Failed to set template")
        .tick_strings(&["⠋", "⠙", "⠴", "⠦", "⠇", "⠸", "⠼", "⠴", "⠦", "⠇", "⠸"]));

    let file_pb = mp.add(ProgressBar::new(0));
    file_pb.set_style(ProgressStyle::default_bar()
        .template("{prefix:.bold.dim} {wide_bar:.cyan/blue} {pos}")
        .expect("Failed to set template"));
    file_pb.set_prefix("Files:");

    let dir_pb = mp.add(ProgressBar::new(0));
    dir_pb.set_style(ProgressStyle::default_bar()
        .template("{prefix:.bold.dim} {wide_bar:.magenta/blue} {pos}")
        .expect("Failed to set template"));
    dir_pb.set_prefix("Dirs: ");

    let size_pb = mp.add(ProgressBar::new(0));
    size_pb.set_style(ProgressStyle::default_bar()
        .template("{prefix:.bold.dim} {wide_bar:.yellow/blue} {bytes}")
        .expect("Failed to set template"));
    size_pb.set_prefix("Size: ");

    spinner.set_message("Processing...");
    visit_dirs(Path::new(path), &file_pb, &dir_pb, &size_pb).expect("Error reading directory");
    spinner.finish_with_message("Done!");

    let duration = start_time.elapsed();

    println!("\n{}", format!("Total size: {}", ByteSize(size_pb.position())).cyan());
    println!("{}", format!("File count: {}", file_pb.position()).cyan());
    println!("{}", format!("Directory count: {}", dir_pb.position()).cyan());
    println!("{}", format!("Execution time: {:.2?}", duration).cyan());
}

fn visit_dirs(dir: &Path, file_pb: &ProgressBar, dir_pb: &ProgressBar, size_pb: &ProgressBar) -> std::io::Result<()> {
    if dir.is_dir() {
        dir_pb.inc(1);
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, file_pb, dir_pb, size_pb)?;
            } else {
                file_pb.inc(1);
                if let Ok(metadata) = fs::metadata(&path) {
                    size_pb.inc(metadata.len());
                }
            }
        }
    }
    Ok(())
}
