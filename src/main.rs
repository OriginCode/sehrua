use anyhow::Result;

use repl::start_repl;

mod eval;
mod repl;

fn main() -> Result<()> {
    start_repl()
}
