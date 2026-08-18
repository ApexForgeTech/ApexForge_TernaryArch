use super::super::flags::Flags;
#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn new_flags_all_false() {
        let f = Flags::new();
        assert!(!f.zero);
        assert!(!f.carry);
        assert!(!f.overflow);
        assert!(!f.negative);
    }
 
    #[test]
    fn clear_resets_all() {
        let mut f = Flags::new();
        f.zero  = true;
        f.carry = true;
        f.clear();
        assert!(!f.zero);
        assert!(!f.carry);
    }
 
    #[test]
    fn update_after_add_zero_result() {
        let mut f = Flags::new();
        f.update_after_add(true, false); // nəticə 0, overflow yox
        assert!(f.zero);
        assert!(!f.carry);
    }
 
    #[test]
    fn update_after_add_overflow() {
        let mut f = Flags::new();
        f.update_after_add(false, true); // overflow var
        assert!(!f.zero);
        assert!(f.carry);
        assert!(f.overflow);
    }
 
    #[test]
    fn update_after_sub_underflow() {
        let mut f = Flags::new();
        f.update_after_sub(false, true); // underflow
        assert!(f.carry);
        assert!(f.negative);
    }
 
    #[test]
    fn display_format() {
        let f = Flags::new();
        assert_eq!(f.display(), "Z=0 C=0 O=0 N=0");
 
        let mut f2 = Flags::new();
        f2.zero  = true;
        f2.carry = true;
        assert_eq!(f2.display(), "Z=1 C=1 O=0 N=0");
    }
}
 