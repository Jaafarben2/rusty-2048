use crate::game::game_kernel as game_kernel;
use game_kernel::{RetainerManager, CtxElementType, IdxType, RetainerManagerElementType};
use std::ops::Add;


#[derive(Debug,Clone,Copy, PartialEq)]
pub enum SpecificElementType<T:Copy+Add<Output=T>+PartialEq> {
    Some(T),
    Block,
    BlockFixed
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RetainerMergerInfo<T:Copy+Add<Output=T>+PartialEq, CtxElementType> {
    Merged((CtxElementType, SpecificElementType<T>),(CtxElementType, SpecificElementType<T>)),
    NotMerged(CtxElementType, SpecificElementType<T>),
    None
}

pub struct RetainerMerger<T:Copy+Add<Output=T>+PartialEq>(IdxType, IdxType, (Option<(CtxElementType, SpecificElementType<T>)>, Option<(CtxElementType, SpecificElementType<T>)>));

impl<T : Copy+Add<Output=T>+PartialEq> RetainerManager<SpecificElementType<T>> for RetainerMerger<T> {
    
    type RetainerMergerInfoType = RetainerMergerInfo<T, CtxElementType>;

    fn new() -> RetainerMerger<T> {
        RetainerMerger(0,0,(None, None))
    }

    fn push_and_pop_if_filled(&mut self, ctx_element: CtxElementType, element: Option<SpecificElementType<T>>) ->  (RetainerManagerElementType<SpecificElementType<T>>,Self::RetainerMergerInfoType) {
        let ret = match (self.2.0, self.2.1) {
            (Some((ctx_element_a, SpecificElementType::Some(a))), Some((ctx_element_b, SpecificElementType::Some(b)))) if a == b => {
                self.2.0 = None;
                self.2.1 = element.map(|e| (ctx_element, e));
                self.1 +=1;
                (RetainerManagerElementType::GameElement(SpecificElementType::Some(a+a)),RetainerMergerInfo::Merged((ctx_element_a, SpecificElementType::Some(a)), (ctx_element_b, SpecificElementType::Some(b))))
            }
            (Some((ctx_element_a, SpecificElementType::Some(a))), Some(_)) => {
                self.2.0 = self.2.1;
                self.2.1 = element.map(|e| (ctx_element, e));
                self.1 +=1;
                (RetainerManagerElementType::GameElement(SpecificElementType::Some(a)),RetainerMergerInfo::NotMerged(ctx_element_a, SpecificElementType::Some(a)))
            }
            (Some((ctx_element_a, SpecificElementType::Block)), _) => {
                self.2.0 = self.2.1;
                self.2.1 = element.map(|e: SpecificElementType<T>| (ctx_element, e));
                self.1 +=1;
                (RetainerManagerElementType::GameElement(SpecificElementType::Block),RetainerMergerInfo::NotMerged(ctx_element_a, SpecificElementType::Block))
            }
            (Some((ctx_element_a, SpecificElementType::BlockFixed)),_) => {
                self.2.0 = self.2.1;
                self.2.1 = element.map(|e: SpecificElementType<T>| (ctx_element, e));

                let steps = (self.0 - 1 - 1) - self.1;
                
                self.1 = self.0 - 1-1;

                self.1 += 1;
                (RetainerManagerElementType::GameElementWithIncrementalIgnore(steps, SpecificElementType::BlockFixed),RetainerMergerInfo::NotMerged(ctx_element_a, SpecificElementType::BlockFixed))
            }
            (None, Some(_)) => {
                self.2.0 = self.2.1;
                self.2.1 = element.map(|e| (ctx_element, e));
                //self.1 += 1;
                (RetainerManagerElementType::NeutralIgnore,RetainerMergerInfo::None)
            }
            _ => {
                self.2.1 = element.map(|e| (ctx_element, e));
                (RetainerManagerElementType::NeutralIgnore,RetainerMergerInfo::None)
            }
        };
        self.0 +=1;
        ret
    }

    fn pop(&mut self) -> (RetainerManagerElementType<SpecificElementType<T>>,Self::RetainerMergerInfoType) {
        let ret = match (self.2.0, self.2.1) {
            (None, Some((ctx_element_a, SpecificElementType::BlockFixed))) => {
                self.2.1 = None;
                let steps = (self.0 -1) - self.1;
                self.1 = self.0;
                self.1 += 1;
                (RetainerManagerElementType::GameElementWithIncrementalIgnore(steps, SpecificElementType::BlockFixed),RetainerMergerInfo::NotMerged(ctx_element_a, SpecificElementType::BlockFixed))
            }
            (None, Some((ctx_element_b, specific_element_type))) => {
                self.2.1 = None;
                self.1 +=1;
                (RetainerManagerElementType::GameElement(specific_element_type),RetainerMergerInfo::NotMerged(ctx_element_b,specific_element_type))

            }
            (Some((ctx_element_a, SpecificElementType::Some(a))), Some((ctx_element_b, SpecificElementType::Some(b)))) if a==b => {
                self.2.0 = None;
                self.2.1 = None;
                self.1 +=1;
                (RetainerManagerElementType::GameElement(SpecificElementType::Some(a+a)),RetainerMergerInfo::Merged((ctx_element_a, SpecificElementType::Some(a)), (ctx_element_b, SpecificElementType::Some(b))))
            }
            (Some((ctx_element_a, SpecificElementType::BlockFixed)),_) => {
                self.2.0 = self.2.1;
                self.2.1 = None;
                let steps = (self.0 - 1 - 1) - self.1;
                self.1 = self.0 - 1 -1 ;
                self.1 += 1;
                (RetainerManagerElementType::GameElementWithIncrementalIgnore(steps, SpecificElementType::BlockFixed),RetainerMergerInfo::NotMerged(ctx_element_a, SpecificElementType::BlockFixed))
            }
            (Some((ctx_element_a, specific_element_type_a)), Some((ctx_element_b, specific_element_type_b))) => {
                self.2.0 =  Some((ctx_element_b, specific_element_type_b));
                self.2.1 = None;
                self.1 +=1;
                (RetainerManagerElementType::GameElement(specific_element_type_a),RetainerMergerInfo::NotMerged(ctx_element_a, specific_element_type_a))
            }
            (Some((ctx_element_a, specific_element_type)), None) => {
                self.2.0 =  None;
                self.2.1 = None;
                self.1 +=1;
                (RetainerManagerElementType::GameElement(specific_element_type),RetainerMergerInfo::NotMerged(ctx_element_a, specific_element_type))
            }
            _ => { 
                self.2.0 = self.2.1;
                self.2.1 = None;
                (RetainerManagerElementType::NeutralIgnore,RetainerMergerInfo::None)
            }
        };
        self.0 +=1;
        ret
    }
}
