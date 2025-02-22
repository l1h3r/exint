use crate::condition::Condition;
use crate::context::Context;

pub fn band(context: &mut Context) {
  context.build("bitwise_band", |mut func| {
    func = func.signature("fn %NAME(a: %TYPE, b: %TYPE) -> %TYPE");
    func = func.statement("a & b");
    func = func.filecheck("load %TYPE, ptr %REG",       Condition::Gt(64));
    func = func.filecheck("load %TYPE, ptr %REG",       Condition::Gt(64));
    func = func.filecheck("and %TYPE %REG, %REG",       Condition::All);
    func = func.filecheck("store %TYPE %REG, ptr %REG", Condition::Gt(64));
    func = func.filecheck("ret void",                   Condition::Gt(64));
    func = func.filecheck("ret %TYPE %REG",             Condition::Le(64));
    func
  });
}

pub fn bor(context: &mut Context) {
  context.build("bitwise_bor", |mut func| {
    func = func.signature("fn %NAME(a: %TYPE, b: %TYPE) -> %TYPE");
    func = func.statement("a | b");
    func = func.filecheck("load %TYPE, ptr %REG",       Condition::Gt(64));
    func = func.filecheck("load %TYPE, ptr %REG",       Condition::Gt(64));
    func = func.filecheck("or %TYPE %REG, %REG",        Condition::All);
    func = func.filecheck("store %TYPE %REG, ptr %REG", Condition::Gt(64));
    func = func.filecheck("ret void",                   Condition::Gt(64));
    func = func.filecheck("ret %TYPE %REG",             Condition::Le(64));
    func
  });
}

pub fn bxor(context: &mut Context) {
  context.build("bitwise_bxor", |mut func| {
    func = func.signature("fn %NAME(a: %TYPE, b: %TYPE) -> %TYPE");
    func = func.statement("a ^ b");
    func = func.filecheck("load %TYPE, ptr %REG",       Condition::Gt(64));
    func = func.filecheck("load %TYPE, ptr %REG",       Condition::Gt(64));
    func = func.filecheck("xor %TYPE %REG, %REG",       Condition::All);
    func = func.filecheck("store %TYPE %REG, ptr %REG", Condition::Gt(64));
    func = func.filecheck("ret void",                   Condition::Gt(64));
    func = func.filecheck("ret %TYPE %REG",             Condition::Le(64));
    func
  });
}

pub fn bnot(context: &mut Context) {
  context.build("bitwise_bnot", |mut func| {
    func = func.signature("fn %NAME(a: %TYPE) -> %TYPE");
    func = func.statement("!a");
    func = func.filecheck("load %TYPE, ptr %REG",       Condition::Gt(64));
    func = func.filecheck("xor %TYPE %REG, -1",         Condition::All);
    func = func.filecheck("store %TYPE %REG, ptr %REG", Condition::Gt(64));
    func = func.filecheck("ret void",                   Condition::Gt(64));
    func = func.filecheck("ret %TYPE %REG",             Condition::Le(64));
    func
  });
}
