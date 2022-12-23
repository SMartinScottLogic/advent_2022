use anyhow::Result;
use day23::{ResultType, Solution};

fn main() -> Result<()> {
    env_logger::init();

    utils::run::<Solution, ResultType>(&["sample2", "sample"], &["full"])
}