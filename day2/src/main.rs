use anyhow::Result;
use day2::load;
use log::info;
use yansi::Paint;

fn main() -> Result<()> {
    env_logger::init();

    let mut solution = load("input/day2.full")?;
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
