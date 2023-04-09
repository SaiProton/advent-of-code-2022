use crate::day5::Stack;

pub trait Poppable {
    fn try_pop(&mut self, err_msg: &str) -> char;
}

impl Poppable for Stack {
    fn try_pop(&mut self, err_msg: &str) -> char {
        self.pop()
            .map_or_else(|| panic!("{}", err_msg), |popped| popped)
    }
}
