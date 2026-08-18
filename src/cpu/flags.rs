// src/cpu/flags.rs

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Flags {
    pub zero: bool,
    pub carry: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl Flags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn update_zero(&mut self, result_is_zero: bool) {
        self.zero = result_is_zero;
    }

    pub fn update_carry(&mut self, has_carry: bool) {
        self.carry = has_carry;
    }

    pub fn update_overflow(&mut self, has_overflow: bool) {
        self.overflow = has_overflow;
    }

    pub fn update_after_add(&mut self, result_zero: bool, carry_out: bool) {
        self.zero = result_zero;
        self.carry = carry_out;
        self.overflow = carry_out;
        self.negative = false;
    }

    pub fn update_after_sub(&mut self, result_zero: bool, borrow_out: bool) {
        self.zero = result_zero;
        self.carry = borrow_out;
        self.overflow = borrow_out;
        self.negative = borrow_out;
    }

    pub fn display(&self) -> String {
        format!(
            "Z={} C={} O={} N={}",
            self.zero as u8,
            self.carry as u8,
            self.overflow as u8,
            self.negative as u8,
        )
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.display())
    }
}