use anyhow::Result;
use day8::Solution;
use log::{error, info};
use utils::{load, BaseName, Solution as UtilsSolution};
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
    if let Err(e) = run(&filename, false) {
        error!("Failed running against '{filename}': {e:?}");
    }
    let filename = format!("input/{basename}.full");
    if let Err(e) = run(&filename, true) {
        error!("Failed running against '{filename}': {e:?}");
    }
    Ok(())
}

fn run(filename: &str, is_full: bool) -> Result<()> {
    let mut solution = load::<Solution>(filename)?;
    info!(
        "{}{}: {:?}",
        Paint::masked("🎄 "),
        Paint::bold(Paint::yellow("solution")),
        solution
    );
    solution.analyse(is_full);
    info!(
        "{}part1 answer is {}",
        Paint::masked("🎅 "),
        Paint::bold(Paint::red(solution.answer_part1(is_full)?))
    );
    info!(
        "{}part2 answer is {}",
        Paint::masked("🎅 "),
        Paint::bold(Paint::red(solution.answer_part2(is_full)?))
    );

    Ok(())
}
