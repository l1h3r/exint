use crate::condition::Condition;
use crate::context::Context;

static STD: &[usize] = &[1, 2, 4, 8, 16];

pub fn swap1(context: &mut Context) {
  context.build("convert_swap1", |mut func| {
    func = func.signature("fn %NAME(a: %TYPE) -> %TYPE");
    func = func.statement("a.reverse_bits()");
    func = func.filecheck("load %TYPE, ptr %REG",               Condition::Gt(64));
    func = func.filecheck("@llvm.bitreverse.%TYPE(%TYPE %REG)", Condition::All);
    func = func.filecheck("store %TYPE %REG, ptr %REG",         Condition::Gt(64));
    func = func.filecheck("ret void",                           Condition::Gt(64));
    func = func.filecheck("ret %TYPE %REG",                     Condition::Le(64));
    func
  });
}

pub fn swap8(context: &mut Context) {
  context.build("convert_swap8", |mut func| {
    func = func.skip_emit(Condition::Func(|kind| kind.bits() % 16 != 0));
    func = func.signature("fn %NAME(a: %TYPE) -> %TYPE");
    func = func.statement("a.swap_bytes()");
    func = func.filecheck("load %TYPE, ptr %REG",          Condition::Gt(64));
    func = func.filecheck("@llvm.bswap.%TYPE(%TYPE %REG)", Condition::All);
    func = func.filecheck("store %TYPE %REG, ptr %REG",    Condition::Gt(64));
    func = func.filecheck("ret void",                      Condition::Gt(64));
    func = func.filecheck("ret %TYPE %REG",                Condition::Le(64));
    func
  });
}

pub fn rotl(context: &mut Context) {
  context.build("convert_rotl", |mut func| {
    func = func.skip_emit(Condition::Func(|kind| !STD.contains(&kind.bytes()))); // TODO: FIXME
    func = func.signature("fn %NAME(a: %TYPE, b: u32) -> %TYPE");
    func = func.statement("a.rotate_left(b)");
    func = func.filecheck("load %TYPE, ptr %REG",                                 Condition::Gt(64));
    func = func.filecheck("zext i32 %REG to %TYPE",                               Condition::Gt(32));
    func = func.filecheck("trunc i32 %REG to %TYPE",                              Condition::Lt(32));
    func = func.filecheck("@llvm.fshl.%TYPE(%TYPE %REG, %TYPE %REG, %TYPE %REG)", Condition::All);
    func = func.filecheck("store %TYPE %REG, ptr %REG",                           Condition::Gt(64));
    func = func.filecheck("ret void",                                             Condition::Gt(64));
    func = func.filecheck("ret %TYPE %REG",                                       Condition::Le(64));
    func
  });
}

pub fn rotr(context: &mut Context) {
  context.build("convert_rotr", |mut func| {
    func = func.skip_emit(Condition::Func(|kind| !STD.contains(&kind.bytes()))); // TODO: FIXME
    func = func.signature("fn %NAME(a: %TYPE, b: u32) -> %TYPE");
    func = func.statement("a.rotate_right(b)");
    func = func.filecheck("load %TYPE, ptr %REG",                                 Condition::Gt(64));
    func = func.filecheck("zext i32 %REG to %TYPE",                               Condition::Gt(32));
    func = func.filecheck("trunc i32 %REG to %TYPE",                              Condition::Lt(32));
    func = func.filecheck("@llvm.fshr.%TYPE(%TYPE %REG, %TYPE %REG, %TYPE %REG)", Condition::All);
    func = func.filecheck("store %TYPE %REG, ptr %REG",                           Condition::Gt(64));
    func = func.filecheck("ret void",                                             Condition::Gt(64));
    func = func.filecheck("ret %TYPE %REG",                                       Condition::Le(64));
    func
  });
}
