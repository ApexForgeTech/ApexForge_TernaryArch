#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(u8)]
pub enum Trit {
    Zero=0,
    One=1,
    Two=2
}
impl Trit {
    pub const BASE: u8 = 3;

    #[inline]
    pub const fn value(self) -> u8 {
        self as u8
    }

    #[inline]
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Trit::Zero),
            1 => Some(Trit::One),
            2 => Some(Trit::Two),
            _ => None
        }
    }

    #[inline]
    pub const fn is_zero(self) -> bool {
        matches!(self, Trit::Zero)
    }

    #[inline]
    pub const fn is_nonzero(self) -> bool {
        !self.is_zero()
    }
}