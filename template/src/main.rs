use anyhow::Result;
use log::{error, info};
use template::load;
use utils::BaseName;
use yansi::Paint;

fn main() -> Result<()> {
    env_logger::init();

    let basename = std::env::current_exe()
        .ok()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .map(|s| s.base_name())
        .unwrap()
        .to_owned();

    let filename = format!("input/{basename}.sample");
    if let Err(e) = run(&filename) {
        error!("Failed running against '{filename}': {e:?}");
    }
    let filename = format!("input/{basename}.full");
    if let Err(e) = run(&filename) {
        error!("Failed running against '{filename}': {e:?}");
    }
    Ok(())
}

fn run(filename: &str) -> Result<()> {
    env_logger::init();

    let mut solution = load(filename)?;
    info!(
        "{}{}: {:?}",
        Paint::masked("🎄 "),
        Paint::bold(Paint::yellow("solution")),
        solution
    );
    solution.analyse();
    info!(
        "{}part1 answer is {}",
        Paint::masked("🎅 "),
        Paint::bold(Paint::red(solution.answer_part1()?))
    );
    info!(
        "{}part2 answer is {}",
        Paint::masked("🎅 "),
        Paint::bold(Paint::red(solution.answer_part2()?))
    );

    Ok(())
}
