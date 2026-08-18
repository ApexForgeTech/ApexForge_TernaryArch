// src/cpu/registers.rs

use super::word::TernaryWord;

pub const GENERAL_REGISTER_COUNT: usize = 8;

#[derive(Debug, Clone)]
pub struct Registers {
    pub r0: TernaryWord,
    pub r1: TernaryWord,
    pub r2: TernaryWord,
    pub r3: TernaryWord,
    pub r4: TernaryWord,
    pub r5: TernaryWord,
    pub r6: TernaryWord,
    pub r7: TernaryWord,
    pub pc: TernaryWord,
    pub sp: TernaryWord,
    pub fp: TernaryWord,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            r0: TernaryWord::zero(),
            r1: TernaryWord::zero(),
            r2: TernaryWord::zero(),
            r3: TernaryWord::zero(),
            r4: TernaryWord::zero(),
            r5: TernaryWord::zero(),
            r6: TernaryWord::zero(),
            r7: TernaryWord::zero(),
            pc: TernaryWord::zero(),
            sp: TernaryWord::zero(),
            fp: TernaryWord::zero(),
        }
    }

    pub fn read_general(&self, index: usize) -> Option<&TernaryWord> {
        match index {
            0 => Some(&self.r0),
            1 => Some(&self.r1),
            2 => Some(&self.r2),
            3 => Some(&self.r3),
            4 => Some(&self.r4),
            5 => Some(&self.r5),
            6 => Some(&self.r6),
            7 => Some(&self.r7),
            _ => None,
        }
    }

    pub fn write_general(
        &mut self,
        index: usize,
        value: TernaryWord,
    ) -> Result<(), String> {
        match index {
            0 => self.r0 = value,
            1 => self.r1 = value,
            2 => self.r2 = value,
            3 => self.r3 = value,
            4 => self.r4 = value,
            5 => self.r5 = value,
            6 => self.r6 = value,
            7 => self.r7 = value,
            _ => {
                return Err(format!(
                    "Invalid general register index: {} (valid range: 0..={})",
                    index,
                    GENERAL_REGISTER_COUNT - 1
                ))
            }
        }
        Ok(())
    }

    pub fn read_pc(&self) -> &TernaryWord {
        &self.pc
    }

    pub fn set_pc(&mut self, addr: TernaryWord) {
        self.pc = addr;
    }

    pub fn increment_pc(&mut self) {
        let one = TernaryWord::from_u64(1);
        let result = TernaryWord::add(&self.pc, &one);
        self.pc = result.sum;
    }

    pub fn read_sp(&self) -> &TernaryWord {
        &self.sp
    }

    pub fn set_sp(&mut self, addr: TernaryWord) {
        self.sp = addr;
    }

    pub fn push_sp(&mut self) {
        let one = TernaryWord::from_u64(1);
        let result = TernaryWord::sub(&self.sp, &one);
        self.sp = result.diff;
    }

    pub fn pop_sp(&mut self) {
        let one = TernaryWord::from_u64(1);
        let result = TernaryWord::add(&self.sp, &one);
        self.sp = result.sum;
    }

    pub fn read_fp(&self) -> &TernaryWord {
        &self.fp
    }

    pub fn set_fp(&mut self, addr: TernaryWord) {
        self.fp = addr;
    }

    pub fn dump(&self) {
        let regs = [
            ("R0", &self.r0),
            ("R1", &self.r1),
            ("R2", &self.r2),
            ("R3", &self.r3),
            ("R4", &self.r4),
            ("R5", &self.r5),
            ("R6", &self.r6),
            ("R7", &self.r7),
        ];
        for (name, reg) in &regs {
            println!("{}: {} ({})", name, reg.dump(), reg.to_u64());
        }
        println!("PC: {} ({})", self.pc.dump(), self.pc.to_u64());
        println!("SP: {} ({})", self.sp.dump(), self.sp.to_u64());
        println!("FP: {} ({})", self.fp.dump(), self.fp.to_u64());
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}