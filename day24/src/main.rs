use anyhow::Result;
use day24::{ResultType, Solution};

fn main() -> Result<()> {
    env_logger::init();

    utils::run::<Solution, ResultType>(&["sample", "sample2"], &["full"])
}
