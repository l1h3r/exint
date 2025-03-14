macro_rules! implement {
  ($name:ident) => {
    impl<const N: usize> ::core::hash::Hash for $crate::$name<N> {
      #[inline]
      fn hash<H>(&self, state: &mut H)
      where
        H: ::core::hash::Hasher,
      {
        state.write(&self.to_ne_bytes());
      }

      #[inline]
      fn hash_slice<H>(data: &[Self], state: &mut H)
      where
        H: ::core::hash::Hasher,
      {
        let len: usize = ::core::mem::size_of_val(data);
        let ptr: *const u8 = data.as_ptr().cast();

        // SAFETY: `ptr` is valid and aligned; the underlying bytes of the
        //         numeric types used here do not have any padding.
        state.write(unsafe { ::core::slice::from_raw_parts(ptr, len) });
      }
    }
  };
}

implement!(int);
implement!(uint);
