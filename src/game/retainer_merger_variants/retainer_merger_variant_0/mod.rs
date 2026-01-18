use crate::game::game_kernel as game_kernel;
use game_kernel::{RetainerManager, CtxElementType};
use std::ops::Add;

#[derive(Copy, Clone, Debug)]
pub enum RetainerMergerInfo<ElementType, CtxElementType> {
    Merged((CtxElementType, ElementType),(CtxElementType, ElementType)),
    NotMerged(CtxElementType, ElementType),
}

pub struct RetainerMerger<ElementType:Copy+Add<Output=ElementType>+PartialEq>(Option<(CtxElementType, ElementType)>, Option<(CtxElementType, ElementType)>);

impl<ElementType : Copy+Add<Output=ElementType>+PartialEq> RetainerManager<ElementType> for RetainerMerger<ElementType> {
    
    type RetainerMergerInfoType = RetainerMergerInfo<ElementType, CtxElementType>;

    fn new() -> RetainerMerger<ElementType> {
        RetainerMerger(None, None)
    }

    fn push_and_pop_if_filled(&mut self, ctx_element: CtxElementType, element: Option<ElementType>) -> Option<(ElementType, Self::RetainerMergerInfoType)> {
        match (self.0, self.1) {
            (Some((ctx_element_a, a)), Some((ctx_element_b, b))) if a == b => {
                //println!("{}, {}", a, b);
                self.0 = None;
                self.1 = element.map(|e| (ctx_element, e));
                Some((a+a, RetainerMergerInfo::Merged((ctx_element_a, a), (ctx_element_b, b))))
            }
            (Some((ctx_element_a, a)), Some((ctx_element_b, b))) => {
                self.0 = self.1;
                self.1 = element.map(|e| (ctx_element, e));
                Some((a, RetainerMergerInfo::NotMerged(ctx_element_a, a)))
            }
            (None, Some((ctx_element_b, b))) => {
                self.0 = self.1;
                self.1 = element.map(|e| (ctx_element, e));
                None
            }
            _ => {
                self.1 = element.map(|e| (ctx_element, e));
                None
            }
        }
    }
    fn pop(&mut self) -> Option<(ElementType, Self::RetainerMergerInfoType)> {
        match (self.0, self.1) {
            (None, Some((ctx_element_b, b))) => {
                self.1 = None;
                Some((b, RetainerMergerInfo::NotMerged(ctx_element_b, b)))
            }
            (Some((ctx_element_a, a)), Some((ctx_element_b, b))) if a==b => {
                self.0 = None;
                self.1 = None;
                Some((a+a, RetainerMergerInfo::Merged((ctx_element_a, a), (ctx_element_b, b))))
            }
            (Some((ctx_element_a, a)), Some((ctx_element_b, b))) => {
                self.0 =  Some((ctx_element_b, b));
                self.1 = None;
                Some((a, RetainerMergerInfo::NotMerged(ctx_element_a, a)))
            }
            (Some((ctx_element_a, a)), None) => {
                self.0 =  None;
                self.1 = None;
                Some((a, RetainerMergerInfo::NotMerged(ctx_element_a, a)))
            }
            _ => { 
                self.0 = self.1;
                self.1 = None;
                None }
        }

    }
}
