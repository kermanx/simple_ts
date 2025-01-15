use bitflags::bitflags;

bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct Facts: u32 {
    const NONE = 0;

    const T_EQ_STRING = 1 << 0;
    const T_EQ_NUMBER = 1 << 1;
    const T_EQ_BIGINT = 1 << 2;
    const T_EQ_BOOLEAN = 1 << 3;
    const T_EQ_SYMBOL = 1 << 4;
    const T_EQ_OBJECT = 1 << 5;
    const T_EQ_FUNCTION = 1 << 6;

    const T_NE_STRING = 1 << 7;
    const T_NE_NUMBER = 1 << 8;
    const T_NE_BIGINT = 1 << 9;
    const T_NE_BOOLEAN = 1 << 10;
    const T_NE_SYMBOL = 1 << 11;
    const T_NE_OBJECT = 1 << 12;
    const T_NE_FUNCTION = 1 << 13;

    const EQ_NULL = 1 << 14;
    const EQ_UNDEFINED = 1 << 15;

    const NE_NULL = 1 << 16;
    const NE_UNDEFINED = 1 << 17;

    const IS_NULLISH = 1 << 18;
    const NOT_NULLISH = 1 << 19;

    const TRUTHY = 1 << 20;
    const FALSY = 1 << 21;

    const T_NE_ALL = Self::T_NE_STRING.bits()
      | Self::T_NE_NUMBER.bits()
      | Self::T_NE_BIGINT.bits()
      | Self::T_NE_BOOLEAN.bits()
      | Self::T_NE_SYMBOL.bits()
      | Self::T_NE_OBJECT.bits()
      | Self::T_NE_FUNCTION.bits()
      | Self::NE_NULL.bits()
      | Self::NE_UNDEFINED.bits()
      | Self::NOT_NULLISH.bits();
  }
}

impl Facts {
  pub fn truthy(truthy: bool) -> Self {
    if truthy {
      Self::TRUTHY
    } else {
      Self::FALSY
    }
  }
}
