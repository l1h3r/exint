mod impls;
mod loops;

pub(crate) use self::impls::specialize;
pub(crate) use self::loops::cfor;
pub(crate) use self::loops::cmap;
pub(crate) use self::loops::crev;
