use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use itertools::Itertools;
use std::path::Path;
use std::process::{Command, ExitStatus};
use std::time::{Duration, Instant};

fn all_solutions() -> impl Iterator<Item = String> {
    let path = Path::new("./src/bin");
    path.read_dir().unwrap().filter_map(|f| f.ok()).map(|f| {
        f.file_name()
            .to_str()
            .unwrap()
            .strip_suffix(".rs")
            .unwrap()
            .to_string()
    })
}

/// Compile the solution, execute it and return the execution time
fn execute_solution(name: &str) -> Result<Duration, ExitStatus> {
    let status = Command::new("cargo")
        .args(["build", "--release", "-q", "--bin", name])
        .status()
        .unwrap();
    if !status.success() {
        return Err(status);
    }
    let start = Instant::now();
    let status = Command::new("cargo")
        .args(["run", "--release", "-q", "--bin", name])
        .status()
        .unwrap();
    match status.success() {
        true => Ok(start.elapsed()),
        false => Err(status),
    }
}

fn main() {
    let filenames = all_solutions().collect_vec();

    let progress = MultiProgress::new();
    let bar = progress.add(ProgressBar::new(filenames.len() as u64));
    let template = "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}";
    bar.set_style(
        ProgressStyle::with_template(template)
            .unwrap()
            .progress_chars("#>-"),
    );
    bar.enable_steady_tick(Duration::from_millis(50));

    let total = filenames
        .iter()
        .map(|name| {
            let spinner = progress.insert_before(
                &bar,
                ProgressBar::new_spinner().with_message(format!("Compile and run {name}")),
            );
            spinner.enable_steady_tick(Duration::from_millis(10));
            let (end_msg, duration) = match execute_solution(name.as_str()) {
                Ok(time) => (
                    format!("{} executed in {} seconds", name, time.as_secs_f32()),
                    time,
                ),
                Err(status) => (
                    format!("{name} failed with status {status}"),
                    Duration::from_secs(0),
                ),
            };
            spinner.finish_with_message(end_msg);
            bar.inc(1);
            duration
        })
        .sum::<Duration>();

    bar.finish_and_clear();
    println!("Total time : {} seconds", total.as_secs_f32());
}
