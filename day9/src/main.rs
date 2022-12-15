use anyhow::Result;
use day9::Solution;

fn main() -> Result<()> {
    env_logger::init();

    utils::run::<Solution, usize>(&["sample", "sample2"], &["full"])
}
