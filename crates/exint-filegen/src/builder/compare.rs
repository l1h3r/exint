use crate::condition::Condition;
use crate::context::Context;

#[rustfmt::skip]
pub fn eq(context: &mut Context) {
  context.build("compare_eq", |mut func| {
    func = func.signature("fn %NAME(a: %TYPE, b: %TYPE) -> bool");
    func = func.statement("a.eq(&b)");
    func = func.filecheck("load %TYPE, ptr %REG", Condition::Gt(64));
    func = func.filecheck("load %TYPE, ptr %REG", Condition::Gt(64));
    func = func.filecheck("icmp eq %TYPE",        Condition::All);
    func = func.filecheck("ret i1 %REG",          Condition::All);
    func
  });
}

#[rustfmt::skip]
pub fn cmp(context: &mut Context) {
  context.build("compare_cmp", |mut func| {
    func = func.signature("fn %NAME(a: %TYPE, b: %TYPE) -> ::core::cmp::Ordering");
    func = func.statement("a.cmp(&b)");
    func = func.filecheck("load %TYPE, ptr %REG",           Condition::Gt(64));
    func = func.filecheck("load %TYPE, ptr %REG",           Condition::Gt(64));
    func = func.filecheck("icmp ult %TYPE",                 Condition::Uint(true));
    func = func.filecheck("icmp slt %TYPE",                 Condition::Uint(false));
    func = func.filecheck("icmp ne %TYPE",                  Condition::All);
    func = func.filecheck("zext i1 %REG to i8",             Condition::All);
    func = func.filecheck("select i1 %REG, i8 -1, i8 %REG", Condition::All);
    func = func.filecheck("ret i8 %REG",                    Condition::All);
    func
  });
}
