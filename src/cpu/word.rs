// src/cpu/word.rs

use super::trit::{Trit, AddResult};
use std::fmt;

pub const WORD_WIDTH: usize = 16;
pub const TRIT_BASE: u64 = 3;
pub const WORD_MAX: u64 = 43_046_720; // 3^16 - 1

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TernaryWord {
    pub trits: [Trit; WORD_WIDTH],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordAddResult {
    pub sum: TernaryWord,
    pub overflow: Trit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordSubResult {
    pub diff: TernaryWord,
    pub underflow: Trit,
}

impl TernaryWord {
    pub const fn zero() -> Self {
        Self {
            trits: [Trit::Zero; WORD_WIDTH],
        }
    }

    pub fn from_u64(mut value: u64) -> Self {
        let mut trits = [Trit::Zero; WORD_WIDTH];
        for trit in trits.iter_mut() {
            if value == 0 {
                break;
            }
            let digit = (value % TRIT_BASE) as u8;
            *trit = Trit::from_u8(digit).unwrap();
            value /= TRIT_BASE;
        }
        Self { trits }
    }

    pub fn to_u64(&self) -> u64 {
        let mut result: u64 = 0;
        let mut power: u64 = 1;
        for &trit in &self.trits {
            result += trit.value() as u64 * power;
            power *= TRIT_BASE;
        }
        result
    }

    pub fn get(&self, index: usize) -> Option<Trit> {
        self.trits.get(index).copied()
    }

    pub fn set(&mut self, index: usize, value: Trit) -> Result<(), String> {
        match self.trits.get_mut(index) {
            Some(slot) => {
                *slot = value;
                Ok(())
            }
            None => Err(format!(
                "Trit index {} out of range (word width is {})",
                index, WORD_WIDTH
            )),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.trits.iter().all(|t| t.is_zero())
    }

    pub fn dump(&self) -> String {
        self.trits
            .iter()
            .rev()
            .map(|t| char::from(b'0' + t.value()))
            .collect()
    }

    pub fn dump_compact(&self) -> String {
        let full = self.dump();
        let trimmed = full.trim_start_matches('0');
        if trimmed.is_empty() {
            "0".to_string()
        } else {
            trimmed.to_string()
        }
    }

    pub fn add(a: &TernaryWord, b: &TernaryWord) -> WordAddResult {
        let mut result = [Trit::Zero; WORD_WIDTH];
        let mut carry = Trit::Zero;
        for i in 0..WORD_WIDTH {
            let r = Trit::add(a.trits[i], b.trits[i], carry);
            result[i] = r.sum;
            carry = r.carry;
        }
        WordAddResult {
            sum: TernaryWord { trits: result },
            overflow: carry,
        }
    }

    pub fn sub(a: &TernaryWord, b: &TernaryWord) -> WordSubResult {
        let mut result = [Trit::Zero; WORD_WIDTH];
        let mut borrow = Trit::Zero;
        for i in 0..WORD_WIDTH {
            let r = Trit::sub(a.trits[i], b.trits[i], borrow);
            result[i] = r.diff;
            borrow = r.borrow;
        }
        WordSubResult {
            diff: TernaryWord { trits: result },
            underflow: borrow,
        }
    }

    pub fn not(&self) -> TernaryWord {
        let mut result = [Trit::Zero; WORD_WIDTH];
        for i in 0..WORD_WIDTH {
            result[i] = self.trits[i].not_trit();
        }
        TernaryWord { trits: result }
    }

    pub fn trit_min(a: &TernaryWord, b: &TernaryWord) -> TernaryWord {
        let mut result = [Trit::Zero; WORD_WIDTH];
        for i in 0..WORD_WIDTH {
            result[i] = Trit::min(a.trits[i], b.trits[i]);
        }
        TernaryWord { trits: result }
    }

    pub fn trit_max(a: &TernaryWord, b: &TernaryWord) -> TernaryWord {
        let mut result = [Trit::Zero; WORD_WIDTH];
        for i in 0..WORD_WIDTH {
            result[i] = Trit::max(a.trits[i], b.trits[i]);
        }
        TernaryWord { trits: result }
    }

    pub fn shift_left(&self, n: usize) -> TernaryWord {
        if n >= WORD_WIDTH {
            return TernaryWord::zero();
        }
        let mut result = [Trit::Zero; WORD_WIDTH];
        for i in n..WORD_WIDTH {
            result[i] = self.trits[i - n];
        }
        TernaryWord { trits: result }
    }

    pub fn shift_right(&self, n: usize) -> TernaryWord {
        if n >= WORD_WIDTH {
            return TernaryWord::zero();
        }
        let mut result = [Trit::Zero; WORD_WIDTH];
        for i in 0..(WORD_WIDTH - n) {
            result[i] = self.trits[i + n];
        }
        TernaryWord { trits: result }
    }
}

impl fmt::Display for TernaryWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}₃ ({}₁₀)", self.dump_compact(), self.to_u64())
    }
}

impl Default for TernaryWord {
    fn default() -> Self {
        Self::zero()
    }
}
