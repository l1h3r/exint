macro_rules! hash {
  ($name:ident) => {
    impl<const S: usize> ::core::hash::Hash for $name<S> {
      #[inline]
      fn hash<H>(&self, state: &mut H)
      where
        H: ::core::hash::Hasher,
      {
        state.write(&Self::to_ne_bytes(*self));
      }

      #[inline]
      fn hash_slice<H>(data: &[Self], state: &mut H)
      where
        H: ::core::hash::Hasher,
      {
        let len: usize = ::core::mem::size_of_val(data);
        let ptr: *const u8 = data.as_ptr().cast();

        // SAFETY: `ptr` is valid and aligned; the underlying bytes of the
        // numeric types used here do not have any padding.
        state.write(unsafe { ::core::slice::from_raw_parts(ptr, len) });
      }
    }
  };
}

pub(crate) use hash;
