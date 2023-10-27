use std::error::Error;

use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    EmitBuilder::builder().git_sha(true).git_describe(true, false, None).emit()?;

    Ok(())
}
