use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::function::Function;
use crate::type_name::TypeName;

// https://llvm.org/docs/CommandGuide/FileCheck.html#filecheck-string-substitution-blocks
// https://llvm.org/docs/LangRef.html#identifiers
static REGISTER: &str = "[[REGISTER:%[-a-zA-Z0-9$._]+]]";

pub(crate) struct Instance {
  pub(crate) name: &'static str,
  pub(crate) data: Vec<Function>,
}

impl Instance {
  pub(crate) fn new<F>(name: &'static str, f: F) -> Self
  where
    F: Fn(Function) -> Function,
  {
    let mut data: Vec<Function> = Vec::with_capacity(32);

    for bytes in 1..=16 {
      data.push(f(Function::new(TypeName::Uint(bytes))));
      data.push(f(Function::new(TypeName::Sint(bytes))));
    }

    data.sort_by_key(|function| function.type_name.bits());
    data.sort_by_key(|function| function.type_name.uint());

    Self { name, data }
  }
}

impl Display for Instance {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    writeln!(f, "#![feature(core_intrinsics)]")?;
    writeln!(f, "#![allow(internal_features)]")?;
    writeln!(f, "use exint_integer::int;")?;
    writeln!(f, "use exint_integer::uint;")?;

    'function: for function in self.data.iter() {
      if let Some(ref condition) = function.skip_emit {
        if condition.accept(&function.type_name) {
          continue 'function;
        }
      }

      let name: String = function.type_name.function(self.name);
      let size: String = function.type_name.bytes().to_string();
      let uint: String = function.type_name.uint().to_string();

      let rust_type: String = function.type_name.rust_type();
      let llvm_type: String = function.type_name.llvm_type();
      let next_type: String = function.type_name.next_type().llvm_type();

      let max_value: String = function.type_name.max_value().to_string();
      let min_value: String = function.type_name.min_value().to_string();

      let signature: String = function
        .signature
        .replace("%NAME", name.as_str())
        .replace("%TYPE", rust_type.as_str());

      writeln!(f)?;
      writeln!(f, "// CHECK-LABEL: @{name}")?;
      writeln!(f, "#[unsafe(no_mangle)]")?;
      writeln!(f, "pub {signature} {{")?;

      'check: for (check, condition) in function.filechecks.iter() {
        if !condition.accept(&function.type_name) {
          continue 'check;
        }

        let content: String = check
          .replace("%REG", REGISTER)
          .replace("%TYPE", llvm_type.as_str())
          .replace("%NEXT", next_type.as_str())
          .replace("%MAXV", max_value.as_str())
          .replace("%MINV", min_value.as_str());

        writeln!(f, "// CHECK: {content}")?;
      }

      for statement in function.statements.iter() {
        let content: String = statement
          .replace("%TYPE", rust_type.as_str())
          .replace("%SIZE", size.as_str())
          .replace("%UINT", uint.as_str());

        writeln!(f, "  {content}")?;
      }

      writeln!(f, "}}")?;
    }

    Ok(())
  }
}
