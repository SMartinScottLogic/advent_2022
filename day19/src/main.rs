use anyhow::Result;
use day19::{ResultType, Solution};

fn main() -> Result<()> {
    env_logger::init();

    utils::run::<Solution, ResultType>(&[], &["full"])
}
