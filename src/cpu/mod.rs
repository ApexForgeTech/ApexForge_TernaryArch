mod registers;
mod trit;
mod word;
mod flags;

mod tests {
	mod test;
	mod flag_test;
    mod register_test;
}
pub use registers::{Registers, GENERAL_REGISTER_COUNT};
pub use trit::{Trit, AddResult, SubResult, MulResult};
pub use word::{TernaryWord, WORD_WIDTH, WORD_MAX};