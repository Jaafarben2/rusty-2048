use crate::game::game_kernel as game_kernel;
use game_kernel::{RetainerManager, CtxElementType, RetainerManagerElementType};
use std::ops::Add;


#[derive(Debug,Clone,Copy, PartialEq)]
pub enum SpecificElementType<T:Copy+Add<Output=T>+PartialEq> {
    Some(T),
    Block,
}

#[derive(Copy, Clone, Debug)]
pub enum RetainerMergerInfo<T:Copy+Add<Output=T>+PartialEq, CtxElementType> {
    Merged((CtxElementType, SpecificElementType<T>),(CtxElementType, SpecificElementType<T>)),
    NotMerged(CtxElementType, SpecificElementType<T>),
    None
}

pub struct RetainerMerger<T:Copy+Add<Output=T>+PartialEq>(Option<(CtxElementType, SpecificElementType<T>)>, Option<(CtxElementType, SpecificElementType<T>)>);

impl<T : Copy+Add<Output=T>+PartialEq> RetainerManager<SpecificElementType<T>> for RetainerMerger<T> {
    
    type RetainerMergerInfoType = RetainerMergerInfo<T, CtxElementType>;

    fn new() -> RetainerMerger<T> {
        RetainerMerger(None, None)
    }

    fn push_and_pop_if_filled(&mut self, ctx_element: CtxElementType, element: Option<SpecificElementType<T>>) ->  (RetainerManagerElementType<SpecificElementType<T>>,Self::RetainerMergerInfoType) {
        match (self.0, self.1) {
            (Some((ctx_element_a, SpecificElementType::Some(a))), Some((ctx_element_b, SpecificElementType::Some(b)))) if a == b => {
                //println!("{}, {}", a, b);
                self.0 = None;
                self.1 = element.map(|e| (ctx_element, e));
                (RetainerManagerElementType::GameElement(SpecificElementType::Some(a+a)),RetainerMergerInfo::Merged((ctx_element_a, SpecificElementType::Some(a)), (ctx_element_b, SpecificElementType::Some(b))))
            }
            (Some((ctx_element_a, SpecificElementType::Some(a))), Some(_)) => {
                self.0 = self.1;
                self.1 = element.map(|e| (ctx_element, e));
                (RetainerManagerElementType::GameElement(SpecificElementType::Some(a)),RetainerMergerInfo::NotMerged(ctx_element_a, SpecificElementType::Some(a)))
            }
            /* block don't need to wait for next  */
            (Some((ctx_element_a, SpecificElementType::Block)), _) => {
                self.0 = self.1;
                self.1 = element.map(|e: SpecificElementType<T>| (ctx_element, e));
                (RetainerManagerElementType::GameElement(SpecificElementType::Block),RetainerMergerInfo::NotMerged(ctx_element_a, SpecificElementType::Block))
            }
            (None, Some(_)) => {
                self.0 = self.1;
                self.1 = element.map(|e| (ctx_element, e));
                (RetainerManagerElementType::NeutralIgnore,RetainerMergerInfo::None)
            }
            _ => {
                self.1 = element.map(|e| (ctx_element, e));
                (RetainerManagerElementType::NeutralIgnore,RetainerMergerInfo::None)
            }
        }
    }
    fn pop(&mut self) -> (RetainerManagerElementType<SpecificElementType<T>>,Self::RetainerMergerInfoType) {
        match (self.0, self.1) {
            (None, Some((ctx_element_b, specific_element_type))) => {
                self.1 = None;
                (RetainerManagerElementType::GameElement(specific_element_type),RetainerMergerInfo::NotMerged(ctx_element_b,specific_element_type))

            }
            (Some((ctx_element_a, SpecificElementType::Some(a))), Some((ctx_element_b, SpecificElementType::Some(b)))) if a==b => {
                self.0 = None;
                self.1 = None;
                (RetainerManagerElementType::GameElement(SpecificElementType::Some(a+a)),RetainerMergerInfo::Merged((ctx_element_a, SpecificElementType::Some(a)), (ctx_element_b, SpecificElementType::Some(b))))
            }
            (Some((ctx_element_a, specific_element_type_a)), Some((ctx_element_b, specific_element_type_b))) => {
                self.0 =  Some((ctx_element_b, specific_element_type_b));
                self.1 = None;
                (RetainerManagerElementType::GameElement(specific_element_type_a),RetainerMergerInfo::NotMerged(ctx_element_a, specific_element_type_a))
            }
            (Some((ctx_element_a, specific_element_type)), None) => {
                self.0 =  None;
                self.1 = None;
                (RetainerManagerElementType::GameElement(specific_element_type),RetainerMergerInfo::NotMerged(ctx_element_a, specific_element_type))
            }
             /* will not occur */
            _ => { 
                self.0 = self.1;
                self.1 = None;
                (RetainerManagerElementType::NeutralIgnore,RetainerMergerInfo::None)
            }
        }

    }
}
