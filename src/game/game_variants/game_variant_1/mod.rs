use dioxus::html::u::is;
use getrandom;
use crate::game::game_kernel as game_kernel;
use game_kernel::{Swap2DGame, Swap2DGameConfig, RetainerManager, BoardIndex};
use crate::game::retainer_merger_variants::retainer_merger_variant_1::RetainerMerger as RetainerMerger;
use crate::game::retainer_merger_variants::retainer_merger_variant_1::{RetainerMergerInfo, SpecificElementType as SpecificElementTypeVariant1 };
use std::ops::Add;

pub use game_kernel::AllowedMoves;
pub use game_kernel::GameStatus;

type T = i32;
pub type SpecificElementType = SpecificElementTypeVariant1<i32>;
pub struct GameVariant<const C_W:usize, const C_H:usize> {
    pub array : [[Option<SpecificElementType>; C_W]; C_H],
    pub mergers_infos : [[Option<<RetainerMerger<T> as RetainerManager<SpecificElementTypeVariant1<T>>>::RetainerMergerInfoType>; C_W]; C_H],
    pub nones_number : usize,
    pub score:i32
}
pub type SpecificGame<const C_W: usize, const C_H: usize> = Swap2DGame<GameVariant<C_W, C_H>>;

impl<const C_W: usize, const C_H: usize> PartialEq for GameVariant<C_W, C_H>{
    fn eq(&self, other :&Self) -> bool {
            for id_x in 0..C_H {
                for id_y in 0..C_W {
                    if self.mergers_infos[id_x][id_y] != other.mergers_infos[id_x][id_y]{
                        return false;
                    }
                }
            }
            true
    }
}

fn can_move<const C_W: usize, const C_H: usize>(g:&SpecificGame<C_W, C_H>, idx :(usize, usize)) -> bool {
    let mut temp_element : Option<SpecificElementType> ;
    let ele = g.board_get_element((idx.0, idx.1));
    if ele == None {
        return true;
    }
    if let BoardIndex::CorrectIndex(Idx) = g.step_2d((idx.0, idx.1), AllowedMoves::DOWN) {
        temp_element = g.board_get_element( (Idx.0, Idx.1));
        if temp_element == None || ((temp_element == ele) && ((temp_element != Some(SpecificElementType::BlockFixed)) && (temp_element != Some(SpecificElementType::Block)))) {
            return true;
        }
    }
    if let BoardIndex::CorrectIndex(Idx) =  g.step_2d((idx.0, idx.1), AllowedMoves::UP)  {
        temp_element = g.board_get_element((Idx.0, Idx.1));
        if temp_element == None || ((temp_element == ele) &&  ((temp_element != Some(SpecificElementType::BlockFixed)) && (temp_element != Some(SpecificElementType::Block))))  {
            return true;
        }    }
    if let BoardIndex::CorrectIndex(Idx) =   g.step_2d((idx.0, idx.1), AllowedMoves::RIGHT)  {
        temp_element = g.board_get_element((Idx.0, Idx.1));
        if temp_element == None ||  ((temp_element == ele) &&  ((temp_element != Some(SpecificElementType::BlockFixed)) && (temp_element != Some(SpecificElementType::Block)))) {
            return true;
        }    }
    if let BoardIndex::CorrectIndex(Idx) =   g.step_2d((idx.0, idx.1), AllowedMoves::LEFT)  {
        temp_element = g.board_get_element((Idx.0, Idx.1));
        if temp_element == None ||  ((temp_element == ele) &&  ((temp_element != Some(SpecificElementType::BlockFixed)) && (temp_element != Some(SpecificElementType::Block))))  {
            return true;
        }
    }
    false
}

// can be optimized later
fn is_status_changed<const C_W: usize, const C_H: usize>(g:&SpecificGame<C_W, C_H>) -> bool {
    for id_x in 0..g.board_size.0 {
        for id_y in 0..g.board_size.1 {
            if let Some(merger_info) = g.game_variant_data.mergers_infos[id_x][id_y] {
                if let  RetainerMergerInfo::Merged((_, _), (_, _)) = merger_info {
                    return true;
                }
                else if let RetainerMergerInfo::NotMerged(OldIdx, _) = merger_info {
                    if OldIdx != (id_x, id_y) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub fn set_nth_none_element<const C_W: usize, const C_H: usize>(g:&mut SpecificGame<C_W, C_H>, insert_idx : usize, ele_set : Option<<SpecificGame<C_W, C_H> as Swap2DGameConfig>::ElementType>){
    let mut curr_idx = 0;
    for id_x in 0..g.board_size.0 {
        for id_y in 0..g.board_size.1 {
            let ele =g.board_get_element( (id_x, id_y));
            if (ele, curr_idx) == (None, insert_idx) {
                g.board_set_element((id_x, id_y), ele_set);
            }
            if ele == None {
                curr_idx += 1;
            }
        }
    }
}

pub fn get_rand_idx(start_idx: usize, end_idx: usize) -> usize {
    //let insert_idx = thread_rng().gen_range(0..g.game_custom_info.nones_number);
    let mut random_value: [u8;1] = [0];
    getrandom::getrandom(&mut random_value);
    (random_value[0] as usize) % (end_idx - start_idx) + start_idx
}

pub fn get_rand_value() -> Option<SpecificElementType>{

    let mut random_value: [u8;1] = [0];

    getrandom::getrandom(&mut random_value);

    // we divide the 256 values [0->255] into 9 (0->8) of 25 ranges + last range 25+6 so
    // Some(4) will have around 12,10 % of proba
    // Some(2) will have around 87,89 % of proba

    if (random_value[0] / 25)  < 9 {
        Some(SpecificElementType::Some(2))
    } else {
        Some(SpecificElementType::Some(2))
    }

}

impl<const W: usize, const H: usize> Swap2DGameConfig for Swap2DGame<GameVariant<W, H>> {

    type ElementType = SpecificElementType;
    type RetainerManager = RetainerMerger<T>;

    fn board_get_element(&self, idx: (usize, usize)) -> Option<Self::ElementType> {
        self.game_variant_data.array[idx.0][idx.1]
    }

    fn board_set_element(&mut self, idx: (usize, usize), element: Option<Self::ElementType>) {
        match (self.game_variant_data.array[idx.0][idx.1], element) {
            (Some(a), None) => self.game_variant_data.nones_number+=1,
            (None, Some(_a)) => self.game_variant_data.nones_number-=1,
            _ => {}
        }
        self.game_variant_data.array[idx.0][idx.1] = element;
    }

    fn board_update_after_move(&mut self, _: AllowedMoves) {
        if self.game_variant_data.nones_number == 0 || !is_status_changed(self) { return;}

        let insert_idx = get_rand_idx(0, self.game_variant_data.nones_number);

        set_nth_none_element(self, insert_idx,get_rand_value());
    }

    fn board_elementary_move_details(&mut self, Idx: (usize, usize), retainer_merger_info: Option<<<Swap2DGame<GameVariant<W, H>> as Swap2DGameConfig>::RetainerManager as RetainerManager<Self::ElementType>>::RetainerMergerInfoType>) {
        self.game_variant_data.mergers_infos[Idx.0][Idx.1] = retainer_merger_info;
        if let  Some(RetainerMergerInfo::Merged((_, SpecificElementType::Some(a)), (_,  SpecificElementType::Some(b)))) = retainer_merger_info {
            self.game_variant_data.score += a + b;
        }
    }

    fn board_game_status_fn(&self) -> GameStatus {
        let mut able_to_move = false;
        for id_x in 0..self.board_size.0 {
            for id_y in 0..self.board_size.1 {
                if self.board_get_element((id_x, id_y)) == Some(SpecificElementType::Some(2048)) {
                    return GameStatus::END_SUCCESS;
                }
                if can_move(self, (id_x, id_y)){
                   able_to_move = true;
                }
            }
        }
        if able_to_move {
            return GameStatus::INPROGRESS;
        }
        GameStatus::END_FAIL
    }
}

impl<const C_W: usize, const C_H: usize> GameVariant<C_W, C_H> {

    // the size shall be under the capacity
    pub fn new_game_specific_dim(dim : (usize, usize)) -> Result<SpecificGame<C_W, C_H>, String>{
        if (dim.0 <= C_W && dim.1 <= C_H) {
            let mut game_variant = GameVariant {
                array: [[None; C_W]; C_H],
                mergers_infos: [[None; C_W]; C_H],
                nones_number: C_W * C_H,
                score : 0,
            };

            let mut g: Swap2DGame<GameVariant<C_W, C_H>> = SpecificGame::game_init(dim, (C_W, C_H), game_variant);

            return Ok(g);
        }
        else {
            Err("The dimension wanted is above the capacity".to_string())
        }
    }

    
    pub fn new_game() -> SpecificGame<C_W, C_H> {
        // here we are sure that width and height are under capacity
        match Self::new_game_specific_dim((C_W, C_H)){
            Ok(g) => g,
            Err(_e) => panic!("This should not happen"),
        }
    }
}


