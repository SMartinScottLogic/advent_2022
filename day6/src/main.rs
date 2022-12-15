use anyhow::Result;
use day6::Solution;

fn main() -> Result<()> {
    env_logger::init();

    utils::run::<Solution, usize>(&["sample"], &["full"])
}
