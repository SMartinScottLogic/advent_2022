use anyhow::Result;
use day5::Solution;

fn main() -> Result<()> {
    env_logger::init();

    utils::run::<Solution, String>(&["sample"], &["full"])
}
