use anyhow::Result;
use day7::Solution;

fn main() -> Result<()> {
    env_logger::init();

    utils::run::<Solution, u64>(&["sample"], &["full"])
}
