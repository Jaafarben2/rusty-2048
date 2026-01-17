use crate::game::game_kernel as game_kernel;
use game_kernel::{RetainerManager};
use std::ops::Add;

pub struct RetainerMerger<ElementType:Copy+Add<Output=ElementType>+PartialEq>(Option<ElementType>, Option<ElementType>);

impl<ElementType : Copy+Add<Output=ElementType>+PartialEq> RetainerManager<ElementType> for RetainerMerger<ElementType> {
    fn new() -> RetainerMerger<ElementType> {
        RetainerMerger(None, None)
    }

    fn push_and_pop_if_filled(&mut self, element: Option<ElementType>) -> Option<ElementType> {
        match (self.0, self.1) {
            (Some(a), Some(b)) if a == b => {
                //println!("{}, {}", a, b);
                self.0 = None;
                self.1 = element;
                Some(a+a)
            }
            (any, Some(b)) => {
                self.0 = self.1;
                self.1 = element;
                any
            }
            _ => {
                self.1 = element;
                None
            }
        }
    }
    fn pop(&mut self) -> Option<ElementType> {
        match (self.0, self.1) {
            (None, Some(b)) => {
                self.1 = None;
                Some(b)
            }
            (Some(a), Some(b)) if a==b => {
                self.0 = None;
                self.1 = None;
                Some(a+a)
            }
            _ => { let temp = self.0;
                self.0 = self.1;
                self.1 = None;
                temp }
        }

    }
}
