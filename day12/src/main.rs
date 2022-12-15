use anyhow::Result;
use day12::{ResultType, Solution};

fn main() -> Result<()> {
    env_logger::init();

    utils::run::<Solution, ResultType>(&["sample"], &["full"])
}
