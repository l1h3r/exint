use crate::condition::Condition;
use crate::context::Context;

pub fn ctpop(context: &mut Context) {
  context.build("inspect_ctpop", |mut func| {
    func = func.signature("fn %NAME(a: %TYPE) -> u32");
    func = func.statement("a.count_ones()");
    func = func.filecheck("load %TYPE, ptr %REG",          Condition::Gt(64));
    func = func.filecheck("@llvm.ctpop.%TYPE(%TYPE %REG)", Condition::All);
    func = func.filecheck("zext %TYPE %REG to i32",        Condition::Lt(32));
    func = func.filecheck("trunc %TYPE %REG to i32",       Condition::Gt(32));
    func = func.filecheck("ret i32 %REG",                  Condition::All);
    func
  });
}

pub fn ctlz(context: &mut Context) {
  context.build("inspect_ctlz", |mut func| {
    func = func.signature("fn %NAME(a: %TYPE) -> u32");
    func = func.statement("a.leading_zeros()");
    func = func.filecheck("load %TYPE, ptr %REG",                   Condition::Gt(64));
    func = func.filecheck("@llvm.ctlz.%TYPE(%TYPE %REG, i1 false)", Condition::All);
    func = func.filecheck("zext %TYPE %REG to i32",                 Condition::Lt(32));
    func = func.filecheck("trunc %TYPE %REG to i32",                Condition::Gt(32));
    func = func.filecheck("ret i32 %REG",                           Condition::All);
    func
  });
}

pub fn cttz(context: &mut Context) {
  context.build("inspect_cttz", |mut func| {
    func = func.signature("fn %NAME(a: %TYPE) -> u32");
    func = func.statement("a.trailing_zeros()");
    func = func.filecheck("load %TYPE, ptr %REG",                   Condition::Gt(64));
    func = func.filecheck("@llvm.cttz.%TYPE(%TYPE %REG, i1 false)", Condition::All);
    func = func.filecheck("zext %TYPE %REG to i32",                 Condition::Lt(32));
    func = func.filecheck("trunc %TYPE %REG to i32",                Condition::Gt(32));
    func = func.filecheck("ret i32 %REG",                           Condition::All);
    func
  });
}

pub fn ctlz_nonzero(context: &mut Context) {
  context.build("inspect_ctlz_nonzero", |mut func| {
    func = func.signature("fn %NAME(a: %TYPE) -> u32");
    func = func.statement("unsafe { ::core::intrinsics::assume(!a.is_zero()) }");
    func = func.statement("a.leading_zeros()");
    func = func.filecheck("load %TYPE, ptr %REG",                  Condition::Gt(64));
    func = func.filecheck("@llvm.ctlz.%TYPE(%TYPE %REG, i1 true)", Condition::All);
    func = func.filecheck("zext %TYPE %REG to i32",                Condition::Lt(32));
    func = func.filecheck("trunc %TYPE %REG to i32",               Condition::Gt(32));
    func = func.filecheck("ret i32 %REG",                          Condition::All);
    func
  });
}

pub fn cttz_nonzero(context: &mut Context) {
  context.build("inspect_cttz_nonzero", |mut func| {
    func = func.signature("fn %NAME(a: %TYPE) -> u32");
    func = func.statement("unsafe { ::core::intrinsics::assume(!a.is_zero()) }");
    func = func.statement("a.trailing_zeros()");
    func = func.filecheck("load %TYPE, ptr %REG",                  Condition::Gt(64));
    func = func.filecheck("@llvm.cttz.%TYPE(%TYPE %REG, i1 true)", Condition::All);
    func = func.filecheck("zext %TYPE %REG to i32",                Condition::Lt(32));
    func = func.filecheck("trunc %TYPE %REG to i32",               Condition::Gt(32));
    func = func.filecheck("ret i32 %REG",                          Condition::All);
    func
  });
}
