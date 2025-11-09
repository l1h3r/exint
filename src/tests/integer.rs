use ::core::cmp::PartialEq;
use ::core::fmt::Debug;
use ::core::iter::Iterator;
use ::core::marker::Sized;
use ::core::mem::size_of;

pub(crate) trait IntExt: Sized + PartialEq + Debug {
  type Iter: Iterator<Item = Self>;

  const MIN: Self;
  const MAX: Self;

  const MIN_STR: &'static str;
  const MAX_STR: &'static str;

  const MIN_STR_OVERFLOW: &'static str;
  const MAX_STR_OVERFLOW: &'static str;

  const BITS: u32;
  const SIZE: usize;

  const MS1: Self;

  const S_1: u32;
  const S_2: u32;
  const S_3: u32;
  const S_4: u32;

  const B_1: Self;
  const B_2: Self;
  const B_3: Self;

  const N_128: Self;
  const N_97: Self;
  const N_20: Self;
  const N_11: Self;
  const N_10: Self;
  const N_2: Self;
  const N_1: Self;

  const N_0: Self;
  const P_0: Self;

  const P_1: Self;
  const P_2: Self;
  const P_3: Self;
  const P_4: Self;
  const P_5: Self;
  const P_6: Self;
  const P_7: Self;
  const P_8: Self;
  const P_9: Self;
  const P_10: Self;
  const P_11: Self;
  const P_12: Self;
  const P_13: Self;
  const P_14: Self;
  const P_15: Self;
  const P_17: Self;
  const P_20: Self;
  const P_24: Self;
  const P_60: Self;
  const P_97: Self;
  const P_120: Self;
  const P_127: Self;
  const P_255: Self;

  const A_5: [Self; 5] = [
    Self::P_1,
    Self::P_2,
    Self::P_3,
    Self::P_4,
    Self::P_5,
  ];

  fn iter() -> Self::Iter;
}

macro_rules! extend {
  (
    @size = $size:literal;
    @min_uint = $min_uint:literal;
    @max_uint = $max_uint:literal;
    @min_sint = $min_sint:literal;
    @max_sint = $max_sint:literal;
  ) => {
    extend!(int<$size>, $min_sint, $max_sint);
    extend!(uint<$size>, $min_uint, $max_uint);
  };
  ($name:ident<$size:literal>, $min:literal, $max:literal) => {
    const _: () = {
      pub(crate) struct Iter($crate::types::$name<$size>, usize);

      impl Iter {
        const SIZE: usize = 1000;
        const TAKE: $crate::types::$name::<$size> = $crate::types::$name::from_usize(Self::SIZE);
        const STEP: $crate::types::$name::<$size> = $crate::types::$name::MAX.const_div(Self::TAKE);

        const fn new() -> Self {
          Self($crate::types::$name::ONE, Self::SIZE)
        }
      }

      impl Iterator for Iter {
        type Item = $crate::types::$name<$size>;

        fn next(&mut self) -> ::core::option::Option<Self::Item> {
          if $size == 1 || self.1 == 0 {
            return ::core::option::Option::None;
          }

          let this: $crate::types::$name<$size> = self.0;

          self.0 = self.0.wrapping_add(Self::STEP);
          self.1 -= 1;

          ::core::option::Option::Some(this)
        }
      }

      impl IntExt for $crate::types::$name<$size> {
        type Iter = Iter;

        const MIN: Self = Self::MIN;
        const MAX: Self = Self::MAX;

        const MIN_STR: &'static str = $min;
        const MAX_STR: &'static str = $max;

        const MIN_STR_OVERFLOW: &str = concat!($min, "0");
        const MAX_STR_OVERFLOW: &str = concat!($max, "0");

        const BITS: u32 = Self::BITS;
        const SIZE: usize = size_of::<Self>();

        const MS1: Self = Self::P_1.checked_shl(Self::BITS - 1).unwrap();

        const S_1: u32 = Self::BITS - 1;
        const S_2: u32 = Self::BITS >> 1;
        const S_3: u32 = Self::BITS >> 3;
        const S_4: u32 = (Self::BITS >> 1) - 1;

        const B_1: Self = Self::from_u8(0b00101100);
        const B_2: Self = Self::from_u8(0b00100001);
        const B_3: Self = Self::from_u8(0b01111001);

        const N_128: Self = Self::from_i8(-128);
        const N_97: Self = Self::from_i8(-97);
        const N_20: Self = Self::from_i8(-20);
        const N_11: Self = Self::from_i8(-11);
        const N_10: Self = Self::from_i8(-10);
        const N_2: Self = Self::from_i8(-2);
        const N_1: Self = Self::from_i8(-1);

        const N_0: Self = Self::from_i8(-0);
        const P_0: Self = Self::from_u8(0);

        const P_1: Self = Self::from_u8(1);
        const P_2: Self = Self::from_u8(2);
        const P_3: Self = Self::from_u8(3);
        const P_4: Self = Self::from_u8(4);
        const P_5: Self = Self::from_u8(5);
        const P_6: Self = Self::from_u8(6);
        const P_7: Self = Self::from_u8(7);
        const P_8: Self = Self::from_u8(8);
        const P_9: Self = Self::from_u8(9);
        const P_10: Self = Self::from_u8(10);
        const P_11: Self = Self::from_u8(11);
        const P_12: Self = Self::from_u8(12);
        const P_13: Self = Self::from_u8(13);
        const P_14: Self = Self::from_u8(14);
        const P_15: Self = Self::from_u8(15);
        const P_17: Self = Self::from_u8(17);
        const P_20: Self = Self::from_u8(20);
        const P_24: Self = Self::from_u8(24);
        const P_60: Self = Self::from_u8(60);
        const P_97: Self = Self::from_u8(97);
        const P_120: Self = Self::from_u8(120);
        const P_127: Self = Self::from_u8(127);
        const P_255: Self = Self::from_u8(255);

        fn iter() -> Self::Iter {
          Iter::new()
        }
      }
    };
  };
}

// -----------------------------------------------------------------------------
// Standard Sizes
// -----------------------------------------------------------------------------

extend! {
  @size = 1;
  @min_uint = "0";
  @max_uint = "255";
  @min_sint = "-128";
  @max_sint = "127";
}

extend! {
  @size = 2;
  @min_uint = "0";
  @max_uint = "65535";
  @min_sint = "-32768";
  @max_sint = "32767";
}

extend! {
  @size = 4;
  @min_uint = "0";
  @max_uint = "4294967295";
  @min_sint = "-2147483648";
  @max_sint = "2147483647";
}

extend! {
  @size = 8;
  @min_uint = "0";
  @max_uint = "18446744073709551615";
  @min_sint = "-9223372036854775808";
  @max_sint = "9223372036854775807";
}

extend! {
  @size = 16;
  @min_uint = "0";
  @max_uint = "340282366920938463463374607431768211455";
  @min_sint = "-170141183460469231731687303715884105728";
  @max_sint = "170141183460469231731687303715884105727";
}

// -----------------------------------------------------------------------------
// Extended Sizes
// -----------------------------------------------------------------------------

extend! {
  @size = 3;
  @min_uint = "0";
  @max_uint = "16777215";
  @min_sint = "-8388608";
  @max_sint = "8388607";
}

extend! {
  @size = 5;
  @min_uint = "0";
  @max_uint = "1099511627775";
  @min_sint = "-549755813888";
  @max_sint = "549755813887";
}

extend! {
  @size = 6;
  @min_uint = "0";
  @max_uint = "281474976710655";
  @min_sint = "-140737488355328";
  @max_sint = "140737488355327";
}

extend! {
  @size = 7;
  @min_uint = "0";
  @max_uint = "72057594037927935";
  @min_sint = "-36028797018963968";
  @max_sint = "36028797018963967";
}

extend! {
  @size = 9;
  @min_uint = "0";
  @max_uint = "4722366482869645213695";
  @min_sint = "-2361183241434822606848";
  @max_sint = "2361183241434822606847";
}

extend! {
  @size = 10;
  @min_uint = "0";
  @max_uint = "1208925819614629174706175";
  @min_sint = "-604462909807314587353088";
  @max_sint = "604462909807314587353087";
}

extend! {
  @size = 11;
  @min_uint = "0";
  @max_uint = "309485009821345068724781055";
  @min_sint = "-154742504910672534362390528";
  @max_sint = "154742504910672534362390527";
}

extend! {
  @size = 12;
  @min_uint = "0";
  @max_uint = "79228162514264337593543950335";
  @min_sint = "-39614081257132168796771975168";
  @max_sint = "39614081257132168796771975167";
}

extend! {
  @size = 13;
  @min_uint = "0";
  @max_uint = "20282409603651670423947251286015";
  @min_sint = "-10141204801825835211973625643008";
  @max_sint = "10141204801825835211973625643007";
}

extend! {
  @size = 14;
  @min_uint = "0";
  @max_uint = "5192296858534827628530496329220095";
  @min_sint = "-2596148429267413814265248164610048";
  @max_sint = "2596148429267413814265248164610047";
}

extend! {
  @size = 15;
  @min_uint = "0";
  @max_uint = "1329227995784915872903807060280344575";
  @min_sint = "-664613997892457936451903530140172288";
  @max_sint = "664613997892457936451903530140172287";
}

// -----------------------------------------------------------------------------
// BigInt Sizes
// -----------------------------------------------------------------------------

extend! {
  @size = 31;
  @min_uint = "0";
  @max_uint = "452312848583266388373324160190187140051835877600158453279131187530910662655";
  @min_sint = "-226156424291633194186662080095093570025917938800079226639565593765455331328";
  @max_sint = "226156424291633194186662080095093570025917938800079226639565593765455331327";
}

extend! {
  @size = 32;
  @min_uint = "0";
  @max_uint = "115792089237316195423570985008687907853269984665640564039457584007913129639935";
  @min_sint = "-57896044618658097711785492504343953926634992332820282019728792003956564819968";
  @max_sint = "57896044618658097711785492504343953926634992332820282019728792003956564819967";
}

extend! {
  @size = 48;
  @min_uint = "0";
  @max_uint = "39402006196394479212279040100143613805079739270465446667948293404245721771497210611414266254884915640806627990306815";
  @min_sint = "-19701003098197239606139520050071806902539869635232723333974146702122860885748605305707133127442457820403313995153408";
  @max_sint = "19701003098197239606139520050071806902539869635232723333974146702122860885748605305707133127442457820403313995153407";
}

extend! {
  @size = 64;
  @min_uint = "0";
  @max_uint = "13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095";
  @min_sint = "-6703903964971298549787012499102923063739682910296196688861780721860882015036773488400937149083451713845015929093243025426876941405973284973216824503042048";
  @max_sint = "6703903964971298549787012499102923063739682910296196688861780721860882015036773488400937149083451713845015929093243025426876941405973284973216824503042047";
}

extend! {
  @size = 128;
  @min_uint = "0";
  @max_uint = "179769313486231590772930519078902473361797697894230657273430081157732675805500963132708477322407536021120113879871393357658789768814416622492847430639474124377767893424865485276302219601246094119453082952085005768838150682342462881473913110540827237163350510684586298239947245938479716304835356329624224137215";
  @min_sint = "-89884656743115795386465259539451236680898848947115328636715040578866337902750481566354238661203768010560056939935696678829394884407208311246423715319737062188883946712432742638151109800623047059726541476042502884419075341171231440736956555270413618581675255342293149119973622969239858152417678164812112068608";
  @max_sint = "89884656743115795386465259539451236680898848947115328636715040578866337902750481566354238661203768010560056939935696678829394884407208311246423715319737062188883946712432742638151109800623047059726541476042502884419075341171231440736956555270413618581675255342293149119973622969239858152417678164812112068607";
}
