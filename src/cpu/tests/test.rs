
use crate::cpu::trit::{Trit, AddResult};
use crate::cpu::word::{TernaryWord, WORD_MAX, WORD_WIDTH};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_zero() {
        let w = TernaryWord::zero();
        assert!(w.is_zero());
        assert_eq!(w.to_u64(), 0);
    }

    #[test]
    fn from_u64_zero() {
        let w = TernaryWord::from_u64(0);
        assert!(w.is_zero());
    }

    #[test]
    fn from_to_u64_round_trip() {
        let values = [0, 1, 2, 3, 4, 5, 9, 27, 42, 100, 1000, WORD_MAX];
        for &v in &values {
            let w = TernaryWord::from_u64(v);
            assert_eq!(w.to_u64(), v, "Round-trip failed for {v}");
        }
    }

    #[test]
    fn from_u64_known_value() {
        let w = TernaryWord::from_u64(5);
        assert_eq!(w.get(0), Some(Trit::Two));
        assert_eq!(w.get(1), Some(Trit::One));
        assert_eq!(w.get(2), Some(Trit::Zero));
    }

    #[test]
    fn word_max_value() {
        let w = TernaryWord::from_u64(WORD_MAX);
        for i in 0..WORD_WIDTH {
            assert_eq!(w.get(i), Some(Trit::Two), "trit[{i}] != Two");
        }
        assert_eq!(w.to_u64(), WORD_MAX);
    }

    #[test]
    fn get_set_round_trip() {
        let mut w = TernaryWord::zero();
        w.set(5, Trit::Two).unwrap();
        assert_eq!(w.get(5), Some(Trit::Two));
        assert_eq!(w.get(4), Some(Trit::Zero));
    }

    #[test]
    fn set_out_of_range_returns_err() {
        let mut w = TernaryWord::zero();
        assert!(w.set(WORD_WIDTH, Trit::One).is_err());
        assert!(w.set(99, Trit::One).is_err());
    }

    #[test]
    fn get_out_of_range_returns_none() {
        let w = TernaryWord::zero();
        assert_eq!(w.get(WORD_WIDTH), None);
    }

    #[test]
    fn dump_zero_is_all_zeros() {
        let w = TernaryWord::zero();
        assert_eq!(w.dump(), "0".repeat(WORD_WIDTH));
    }

    #[test]
    fn dump_compact_zero() {
        let w = TernaryWord::zero();
        assert_eq!(w.dump_compact(), "0");
    }

    #[test]
    fn dump_known() {
        let w = TernaryWord::from_u64(42);
        assert!(w.dump().ends_with("1120"), "dump was: {}", w.dump());
        assert_eq!(w.dump_compact(), "1120");
    }

    #[test]
    fn word_add_simple() {
        let a = TernaryWord::from_u64(42);
        let b = TernaryWord::from_u64(10);
        let r = TernaryWord::add(&a, &b);
        assert_eq!(r.sum.to_u64(), 52);
        assert_eq!(r.overflow, Trit::Zero);
    }

    #[test]
    fn word_add_zero() {
        let a = TernaryWord::from_u64(100);
        let b = TernaryWord::zero();
        let r = TernaryWord::add(&a, &b);
        assert_eq!(r.sum.to_u64(), 100);
    }

    #[test]
    fn word_add_overflow() {
        let a = TernaryWord::from_u64(WORD_MAX);
        let b = TernaryWord::from_u64(1);
        let r = TernaryWord::add(&a, &b);
        assert_eq!(r.sum.to_u64(), 0);
        assert_eq!(r.overflow, Trit::One);
    }

    #[test]
    fn word_add_many_values() {
        let pairs = [(0, 0), (1, 1), (27, 27), (100, 200), (1000, 999)];
        for (a, b) in pairs {
            let wa = TernaryWord::from_u64(a);
            let wb = TernaryWord::from_u64(b);
            let r = TernaryWord::add(&wa, &wb);
            assert_eq!(r.sum.to_u64(), a + b, "ADD {a}+{b} failed");
            assert_eq!(r.overflow, Trit::Zero);
        }
    }

    #[test]
    fn word_sub_simple() {
        let a = TernaryWord::from_u64(52);
        let b = TernaryWord::from_u64(10);
        let r = TernaryWord::sub(&a, &b);
        assert_eq!(r.diff.to_u64(), 42);
        assert_eq!(r.underflow, Trit::Zero);
    }

    #[test]
    fn word_sub_zero() {
        let a = TernaryWord::from_u64(100);
        let b = TernaryWord::zero();
        let r = TernaryWord::sub(&a, &b);
        assert_eq!(r.diff.to_u64(), 100);
    }

    #[test]
    fn word_sub_self() {
        let a = TernaryWord::from_u64(12345);
        let r = TernaryWord::sub(&a, &a);
        assert_eq!(r.diff.to_u64(), 0);
        assert_eq!(r.underflow, Trit::Zero);
    }

    #[test]
    fn word_sub_underflow() {
        let a = TernaryWord::zero();
        let b = TernaryWord::from_u64(1);
        let r = TernaryWord::sub(&a, &b);
        assert_eq!(r.underflow, Trit::One);
        assert_eq!(r.diff.to_u64(), WORD_MAX);
    }

    #[test]
    fn add_then_sub_is_identity() {
        let a = TernaryWord::from_u64(500);
        let b = TernaryWord::from_u64(123);
        let added = TernaryWord::add(&a, &b).sum;
        let restored = TernaryWord::sub(&added, &b).diff;
        assert_eq!(restored.to_u64(), 500);
    }

    #[test]
    fn not_zero_is_max() {
        let w = TernaryWord::zero();
        assert_eq!(w.not().to_u64(), WORD_MAX);
    }

    #[test]
    fn not_double_negation() {
        let w = TernaryWord::from_u64(12345);
        assert_eq!(w.not().not(), w);
    }

    #[test]
    fn shift_left_by_one_is_times_three() {
        let w = TernaryWord::from_u64(7);
        let s = w.shift_left(1);
        assert_eq!(s.to_u64(), 21);
    }

    #[test]
    fn shift_right_by_one_is_divide_three() {
        let w = TernaryWord::from_u64(9);
        let s = w.shift_right(1);
        assert_eq!(s.to_u64(), 3);
    }

    #[test]
    fn shift_left_beyond_width_is_zero() {
        let w = TernaryWord::from_u64(42);
        assert_eq!(w.shift_left(WORD_WIDTH).to_u64(), 0);
    }

    #[test]
    fn shift_right_beyond_width_is_zero() {
        let w = TernaryWord::from_u64(42);
        assert_eq!(w.shift_right(WORD_WIDTH).to_u64(), 0);
    }
}