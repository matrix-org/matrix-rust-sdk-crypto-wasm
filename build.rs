use std::error::Error;

use vergen_gitcl::{Emitter, GitclBuilder};

fn main() -> Result<(), Box<dyn Error>> {
    let gitcl = GitclBuilder::default().sha(true).describe(true, false, None).build()?;
    Emitter::default().add_instructions(&gitcl)?.emit()?;

    Ok(())
}
