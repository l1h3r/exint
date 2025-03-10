macro_rules! define_limbs {
  ($name:ident, $primitive:ty, $wide:ty) => {
    #[repr(transparent)]
    pub(crate) struct $name<const N: usize> {
      data: [u8; N],
    }

    impl<const N: usize> $name<N> {
      /// The size of each limb in bytes.
      const BYTES: usize = ::core::mem::size_of::<$primitive>();

      /// The total number of limbs.
      const COUNT: usize = N / Self::BYTES;

      #[inline]
      const fn new(data: [u8; N]) -> Self {
        assert!(N > 0, "cannot access zero-sized limbs");
        assert!(N % Self::BYTES == 0, "cannot access imbalanced limbs");

        Self { data }
      }

      #[inline]
      const fn new_digit(digit: $primitive) -> Self {
        let mut this: Self = Self::new($crate::traits::Consts::UMIN);
        this.write_lsb(0, digit);
        this
      }

      #[inline]
      const fn get(self) -> [u8; N] {
        self.data
      }

      #[inline]
      const fn ptr(&self, index: usize) -> *const $primitive {
        assert!(index < Self::COUNT, "cannot access out-of-bounds location");

        // SAFETY: We just ensured that `index` points to an in-bounds location.
        unsafe { self.data.as_ptr().cast::<$primitive>().add(index) }
      }

      #[inline]
      const fn mut_ptr(&mut self, index: usize) -> *mut $primitive {
        assert!(index < Self::COUNT, "cannot access out-of-bounds location");

        // SAFETY: We just ensured that `index` points to an in-bounds location.
        unsafe { self.data.as_mut_ptr().cast::<$primitive>().add(index) }
      }

      #[inline]
      const fn offset_lsb(&self, index: usize) -> usize {
        $crate::utils::Index(index * Self::BYTES).lsb::<N>() / Self::BYTES
      }

      #[inline]
      const fn offset_msb(&self, index: usize) -> usize {
        $crate::utils::Index(index * Self::BYTES).msb::<N>() / Self::BYTES
      }

      const fn read_lsb(&self, index: usize) -> $primitive {
        let idx: usize = self.offset_lsb(index);
        let ptr: *const $primitive = self.ptr(idx);

        // SAFETY:
        //   - `ptr` is valid for reads of $primitive
        //   - `ptr` points to a byte array that is always initialized
        unsafe { ::core::ptr::read_unaligned(ptr) }
      }

      const fn read_msb(&self, index: usize) -> $primitive {
        let idx: usize = self.offset_msb(index);
        let ptr: *const $primitive = self.ptr(idx);

        // SAFETY:
        //   - `ptr` is valid for reads of $primitive
        //   - `ptr` points to a byte array that is always initialized
        unsafe { ::core::ptr::read_unaligned(ptr) }
      }

      #[inline]
      const fn write_lsb(&mut self, index: usize, value: $primitive) {
        let idx: usize = self.offset_lsb(index);
        let ptr: *mut $primitive = self.mut_ptr(idx);

        // SAFETY: `ptr` is valid for writes of $primitive
        unsafe { ::core::ptr::write_unaligned(ptr, value) }
      }

      #[inline]
      const fn write_msb(&mut self, index: usize, value: $primitive) {
        let idx: usize = self.offset_msb(index);
        let ptr: *mut $primitive = self.mut_ptr(idx);

        // SAFETY: `ptr` is valid for writes of $primitive
        unsafe { ::core::ptr::write_unaligned(ptr, value) }
      }

      #[inline]
      pub(crate) const fn overflowing_uadd(lhs: [u8; N], rhs: [u8; N]) -> ([u8; N], bool) {
        // TODO: Replace with $primitive::carrying_add once stable
        #[inline]
        const fn carrying_add(a: $primitive, b: $primitive, c: bool) -> ($primitive, bool) {
          let (a, b): ($primitive, bool) = <$primitive>::overflowing_add(a, b);
          let (c, d): ($primitive, bool) = <$primitive>::overflowing_add(a, c as $primitive);

          (c, b | d)
        }

        let lhs: Self = Self::new(lhs);
        let rhs: Self = Self::new(rhs);

        let mut carry: bool = false;
        let mut index: usize = 0;
        let mut value: Self = Self::new($crate::traits::Consts::UMIN);

        while index < Self::COUNT {
          let lhs: $primitive = lhs.read_lsb(index);
          let rhs: $primitive = rhs.read_lsb(index);
          let out: ($primitive, bool) = carrying_add(lhs, rhs, carry);

          value.write_lsb(index, out.0);
          carry = out.1;
          index += 1;
        }

        (value.get(), carry)
      }

      #[inline]
      pub(crate) const fn overflowing_usub(lhs: [u8; N], rhs: [u8; N]) -> ([u8; N], bool) {
        // TODO: Replace with $primitive::borrowing_sub once stable
        #[inline]
        const fn borrowing_sub(a: $primitive, b: $primitive, c: bool) -> ($primitive, bool) {
          let (a, b): ($primitive, bool) = <$primitive>::overflowing_sub(a, b);
          let (c, d): ($primitive, bool) = <$primitive>::overflowing_sub(a, c as $primitive);

          (c, b | d)
        }

        let lhs: Self = Self::new(lhs);
        let rhs: Self = Self::new(rhs);

        let mut carry: bool = false;
        let mut index: usize = 0;
        let mut value: Self = Self::new($crate::traits::Consts::UMIN);

        while index < Self::COUNT {
          let lhs: $primitive = lhs.read_lsb(index);
          let rhs: $primitive = rhs.read_lsb(index);
          let out: ($primitive, bool) = borrowing_sub(lhs, rhs, carry);

          value.write_lsb(index, out.0);
          carry = out.1;
          index += 1;
        }

        (value.get(), carry)
      }

      // TODO: Optimize with Karatsuba
      #[inline]
      pub(crate) const fn overflowing_umul(lhs: [u8; N], rhs: [u8; N]) -> ([u8; N], bool) {
        // TODO: Replace with $primitive::carrying_mul_add once stable
        #[inline]
        const fn carrying_mul_add(
          a: $primitive,
          b: $primitive,
          c: $primitive,
          d: $primitive,
        ) -> ($primitive, $primitive) {
          let wide: $wide = (a as $wide) * (b as $wide) + (c as $wide) + (d as $wide);
          let lo: $primitive = wide as $primitive;
          let hi: $primitive = (wide >> <$primitive>::BITS) as $primitive;

          (lo, hi)
        }

        let lhs: Self = Self::new(lhs);
        let rhs: Self = Self::new(rhs);

        let mut carry1: $primitive;
        let mut carry2: $primitive;
        let mut index1: usize = 0;
        let mut index2: usize;

        let mut value: Self = Self::new($crate::traits::Consts::UMIN);
        let mut overflow: bool = false;

        while index1 < Self::COUNT {
          carry1 = 0;
          index2 = 0;

          while index2 < (Self::COUNT - index1) {
            let output: ($primitive, $primitive);
            let lhs_digit: $primitive = lhs.read_lsb(index2);
            let rhs_digit: $primitive = rhs.read_lsb(index1);

            carry2 = value.read_lsb(index1 + index2);
            output = carrying_mul_add(lhs_digit, rhs_digit, carry1, carry2);

            value.write_lsb(index1 + index2, output.0);

            carry1 = output.1;
            index2 += 1;
          }

          overflow |= carry1 != 0;
          index1 += 1;
        }

        (value.get(), overflow)
      }

      #[inline]
      pub(crate) const fn overflowing_sadd(lhs: [u8; N], rhs: [u8; N]) -> ([u8; N], bool) {
        let out: [u8; N] = Self::wrapping_add(lhs, rhs);
        let out_neg: bool = $crate::utils::is_negative(out);
        let lhs_neg: bool = $crate::utils::is_negative(lhs);
        let rhs_neg: bool = $crate::utils::is_negative(rhs);

        (out, !(lhs_neg ^ rhs_neg) & (lhs_neg ^ out_neg))
      }

      #[inline]
      pub(crate) const fn overflowing_ssub(lhs: [u8; N], rhs: [u8; N]) -> ([u8; N], bool) {
        let out: [u8; N] = Self::wrapping_sub(lhs, rhs);
        let out_neg: bool = $crate::utils::is_negative(out);
        let lhs_neg: bool = $crate::utils::is_negative(lhs);
        let rhs_neg: bool = $crate::utils::is_negative(rhs);

        (out, (lhs_neg ^ rhs_neg) & (lhs_neg ^ out_neg))
      }

      #[inline]
      pub(crate) const fn overflowing_smul(lhs: [u8; N], rhs: [u8; N]) -> ([u8; N], bool) {
        let lhs_neg: bool = $crate::utils::is_negative(lhs);
        let rhs_neg: bool = $crate::utils::is_negative(rhs);

        let lhs: [u8; N] = Self::abs(lhs);
        let rhs: [u8; N] = Self::abs(rhs);

        let output: ([u8; N], bool) = Self::overflowing_umul(lhs, rhs);

        if lhs_neg ^ rhs_neg {
          let min: bool = $crate::api::eq(output.0, $crate::traits::Consts::SMIN);
          let out: [u8; N] = Self::neg(output.0);

          (out, output.1 || ($crate::utils::is_negative(output.0) ^ min))
        } else {
          (output.0, output.1 || $crate::utils::is_negative(output.0))
        }
      }

      #[inline]
      pub(crate) const fn wrapping_add(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
        Self::overflowing_uadd(lhs, rhs).0
      }

      #[inline]
      pub(crate) const fn wrapping_sub(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
        Self::overflowing_usub(lhs, rhs).0
      }

      #[inline]
      pub(crate) const fn wrapping_mul(lhs: [u8; N], rhs: [u8; N]) -> [u8; N] {
        Self::overflowing_umul(lhs, rhs).0
      }

      #[inline]
      const fn abs(integer: [u8; N]) -> [u8; N] {
        if $crate::utils::is_negative(integer) {
          Self::wrapping_add($crate::api::bnot(integer), $crate::traits::Consts::ONE)
        } else {
          integer
        }
      }

      #[inline]
      const fn neg(integer: [u8; N]) -> [u8; N] {
        Self::wrapping_sub($crate::traits::Consts::UMIN, integer)
      }

      #[inline]
      const fn find_msb_nz(&self) -> usize {
        let mut index: usize = 0;

        while index < Self::COUNT {
          if self.read_msb(index) != 0 {
            return self.offset_msb(index);
          }

          index += 1;
        }

        0
      }

      #[inline]
      const fn udivrem_wide(lo: $primitive, hi: $primitive, rhs: $primitive) -> ($primitive, $primitive) {
        let wide: $wide = (hi as $wide) << <$primitive>::BITS | lo as $wide;
        let div: $primitive = (wide / rhs as $wide) as $primitive;
        let rem: $primitive = (wide % rhs as $wide) as $primitive;

        (div, rem)
      }

      #[inline]
      const fn udivrem_small(lhs: Self, rhs: $primitive) -> (Self, $primitive) {
        let mut div: Self = Self::new($crate::traits::Consts::UMIN);
        let mut rem: $primitive = 0;

        let mut index: usize = 0;

        while index < Self::COUNT {
          let value: $primitive = lhs.read_msb(index);
          let output: ($primitive, $primitive) = Self::udivrem_wide(value, rem, rhs);

          div.write_msb(index, output.0);
          rem = output.1;

          index += 1;
        }

        (div, rem)
      }

      #[inline]
      pub(crate) const fn udivrem(lhs: [u8; N], rhs: [u8; N]) -> ([u8; N], [u8; N]) {
        match $crate::api::ucmp(lhs, rhs) {
          ::core::cmp::Ordering::Less => {
            ($crate::traits::Consts::UMIN, lhs)
          }
          ::core::cmp::Ordering::Equal => {
            ($crate::traits::Consts::ONE, $crate::traits::Consts::UMIN)
          }
          ::core::cmp::Ordering::Greater => {
            if $crate::api::eq(lhs, $crate::traits::Consts::UMIN) {
              ($crate::traits::Consts::UMIN, $crate::traits::Consts::UMIN)
            } else {
              let lhs: Self = Self::new(lhs);
              let rhs: Self = Self::new(rhs);

              let index: usize = rhs.find_msb_nz();

              if index == 0 {
                let divisor: $primitive = rhs.read_lsb(0);
                let div_rem: (Self, $primitive) = Self::udivrem_small(lhs, divisor);

                (div_rem.0.get(), Self::new_digit(div_rem.1).get())
              } else {
                ::core::panic!(concat!("TODO: ", stringify!($name), "::udivrem"))
              }
            }
          }
        }
      }

      #[inline]
      pub(crate) const fn sdivrem(lhs: [u8; N], rhs: [u8; N]) -> ([u8; N], [u8; N]) {
        let lhs_neg: bool = $crate::utils::is_negative(lhs);
        let rhs_neg: bool = $crate::utils::is_negative(rhs);

        let lhs: [u8; N] = Self::abs(lhs);
        let rhs: [u8; N] = Self::abs(rhs);
        let out: ([u8; N], [u8; N]) = Self::udivrem(lhs, rhs);

        let div: [u8; N] = if lhs_neg ^ rhs_neg {
          Self::neg(out.0)
        } else {
          out.0
        };

        let rem: [u8; N] = if lhs_neg {
          Self::neg(out.1)
        } else {
          out.1
        };

        (div, rem)
      }
    }
  };
}

define_limbs!(Limbs8,  u8,  u16);
define_limbs!(Limbs16, u16, u32);
define_limbs!(Limbs32, u32, u64);
define_limbs!(Limbs64, u64, u128);
