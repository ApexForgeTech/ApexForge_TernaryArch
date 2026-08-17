use std::fmt::write;
use std::result;

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

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct AddResult {
    sum: Trit,
    carry: Trit
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct SubResult {
    diff: Trit,
    borrow: Trit
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct MulResult {
    product: Trit,
    carry: Trit
}

impl Trit {
    pub fn add(a:Trit,b:Trit,carry_in:Trit) -> AddResult{

        let total = a.value() +b.value() +carry_in.value();
        
        AddResult {

             sum: Trit::from_u8(total  % 3).unwrap(), 
             
             carry: Trit::from_u8(total/3).unwrap()
        
        } 
}
pub fn sub(a:Trit,b:Trit,borrow_in:Trit) -> SubResult{
   
   let total:i8= a.value() as i8 - b.value() as i8 - borrow_in.value() as i8;
   
   if total >= 0 {

     SubResult {

         diff: Trit::from_u8((total % 3) as u8 ).unwrap(),
        
        borrow:Trit::Zero
         
         }

   }
   else {
       SubResult {
         diff: Trit::from_u8(((total + 3 ) % 3) as u8).unwrap(),
         borrow: Trit::One
         }
   }
}

pub fn mul(a: Trit, b: Trit) -> MulResult {

    let total=a.value() * b.value();
    MulResult
    { 

        product:Trit::from_u8(total % 3).unwrap(),
        
        carry: Trit::from_u8(total / 3).unwrap()

         }

}


pub fn not_trit(self) -> Trit{
    match self {
        Trit::Zero => Trit::Two,
        Trit::One => Trit::One,
        Trit::Two => Trit::Zero
    }
}

pub fn min(a:Trit,b:Trit) -> Trit {
    if a.value() <= b.value() { a } else { b }
}

pub fn max(a: Trit,b: Trit) -> Trit {
    if a.value() >= b.value() { a } else { b }
}
}


use std::fmt;
 impl fmt::Display for Trit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.value())
    }
 }