use anyhow::Result;
use day8::Solution;

fn main() -> Result<()> {
    env_logger::init();

    utils::run::<Solution, i64>(&["sample"], &["full"])
}
