mod builder;
mod condition;
mod context;
mod function;
mod instance;
mod type_name;

use std::io::Error;
use std::path::Path;

use crate::context::Context;

pub fn build(root: &Path) -> Result<(), Error> {
  let mut context: Context = Context::new(4 + 2 + 4 + 5);

  builder::bitwise::band(&mut context);
  builder::bitwise::bor(&mut context);
  builder::bitwise::bxor(&mut context);
  builder::bitwise::bnot(&mut context);

  builder::compare::eq(&mut context);
  builder::compare::cmp(&mut context);

  builder::convert::swap1(&mut context);
  builder::convert::swap8(&mut context);
  builder::convert::rotl(&mut context);
  builder::convert::rotr(&mut context);

  builder::inspect::ctpop(&mut context);
  builder::inspect::ctlz(&mut context);
  builder::inspect::cttz(&mut context);
  builder::inspect::ctlz_nonzero(&mut context);
  builder::inspect::cttz_nonzero(&mut context);

  context.write(root)?;

  Ok(())
}
