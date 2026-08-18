
#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::word::TernaryWord;
    use crate::cpu::{Registers, GENERAL_REGISTER_COUNT};
 
    #[test]
    fn new_registers_all_zero() {
        let regs = Registers::new();
 
        // All general registers can be zero(0)
        for i in 0..GENERAL_REGISTER_COUNT {
            let r = regs.read_general(i).unwrap();
            assert!(r.is_zero(), "R{i} should be zero at init");
        }
 
        // Special registers don't be zero(0)
        assert!(regs.read_pc().is_zero());
        assert!(regs.read_sp().is_zero());
        assert!(regs.read_fp().is_zero());
    }
 
    #[test]
    fn write_read_general_all() {
        let mut regs = Registers::new();
 
        
        for i in 0..GENERAL_REGISTER_COUNT {
            let val = TernaryWord::from_u64((i as u64 + 1) * 100);
            regs.write_general(i, val.clone()).unwrap();
            let got = regs.read_general(i).unwrap();
            assert_eq!(*got, val, "R{i} write/read mismatch");
        }
    }
 
    #[test]
    fn write_general_invalid_index_returns_err() {
        let mut regs = Registers::new();
        assert!(regs.write_general(8,  TernaryWord::zero()).is_err());
        assert!(regs.write_general(99, TernaryWord::zero()).is_err());
    }
 
    #[test]
    fn read_general_invalid_index_returns_none() {
        let regs = Registers::new();
        assert!(regs.read_general(8).is_none());
        assert!(regs.read_general(usize::MAX).is_none());
    }
 
    #[test]
    fn registers_are_independent() {
        // R2 changing withount touching R3
        let mut regs = Registers::new();
        regs.write_general(2, TernaryWord::from_u64(999)).unwrap();
        assert!(regs.read_general(3).unwrap().is_zero());
        assert_eq!(regs.read_general(2).unwrap().to_u64(), 999);
    }
 
    #[test]
    fn pc_set_and_read() {
        let mut regs = Registers::new();
        regs.set_pc(TernaryWord::from_u64(42));
        assert_eq!(regs.read_pc().to_u64(), 42);
    }
 
    #[test]
    fn pc_increment() {
        let mut regs = Registers::new();
        regs.set_pc(TernaryWord::from_u64(10));
 
        regs.increment_pc();
        assert_eq!(regs.read_pc().to_u64(), 11);
 
        regs.increment_pc();
        assert_eq!(regs.read_pc().to_u64(), 12);
    }
 
    #[test]
    fn sp_push_and_pop() {
        let mut regs = Registers::new();
        regs.set_sp(TernaryWord::from_u64(100));
 
        regs.push_sp(); // SP = 99
        assert_eq!(regs.read_sp().to_u64(), 99);
 
        regs.push_sp(); // SP = 98
        assert_eq!(regs.read_sp().to_u64(), 98);
 
        regs.pop_sp(); // SP = 99
        assert_eq!(regs.read_sp().to_u64(), 99);
    }
 
    #[test]
    fn fp_set_and_read() {
        let mut regs = Registers::new();
        regs.set_fp(TernaryWord::from_u64(500));
        assert_eq!(regs.read_fp().to_u64(), 500);
    }
 
    #[test]
    fn pc_does_not_affect_sp() {
        // Xüsusi registerlər bir-birindən asılı deyil
        let mut regs = Registers::new();
        regs.set_pc(TernaryWord::from_u64(99));
        assert!(regs.read_sp().is_zero());
    }
}
 