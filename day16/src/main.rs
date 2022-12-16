use anyhow::Result;
use day16::{ResultType, Solution};

fn main() -> Result<()> {
    env_logger::init();

    utils::run::<Solution, ResultType>(&["sample"], &["full"])
}
