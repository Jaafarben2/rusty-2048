use std::ops::{Add};


#[derive(Copy, Clone, Debug)]
pub enum AllowedMoves {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum GameStatus {
    INPROGRESS,
    END_FAIL,
    END_SUCCESS,
}

#[derive(Copy, Clone, Debug)]
pub enum BoardIndex {
    CorrectIndex((usize,usize)),
    OutOfBounds
}

pub type CtxElementType = (usize, usize);

pub struct Swap2DGame<GameVariant> {
    board_top_left_corner: (usize, usize),
    pub board_size: (usize, usize),
    board_capacity: (usize, usize),
    pub game_status: GameStatus,
    
    // Here the variant is used to own the data used by the custom implementation
    // to simplify the implementation of the callbacks and have a clean code and 
    // implementation
    // Also a specific implementation can be added here for simplyfication
    pub game_variant_data: GameVariant,
    //_marker: PhantomData<GameVariant>,


}

pub enum RetainerManagerElementType<ElementType>{
    NeutralIgnore,
    IncrementalIgnore,
    GameElement(ElementType)
}

pub trait RetainerManager<ElementType>{
    type RetainerMergerInfoType;
    fn new() -> Self;
    fn push_and_pop_if_filled(&mut self, ctx_element: CtxElementType, element: Option<ElementType>) -> (RetainerManagerElementType<ElementType>,Self::RetainerMergerInfoType);
    fn pop(&mut self) -> (RetainerManagerElementType<ElementType>,Self::RetainerMergerInfoType);
}

pub trait Swap2DGameConfig {

    type ElementType:Copy+PartialEq;
    type RetainerManager:RetainerManager<Self::ElementType>;
    
    fn board_get_element(&self, _: (usize, usize)) -> Option<Self::ElementType>;
    fn board_set_element(&mut self, _: (usize, usize), _: Option<Self::ElementType>);
    fn board_elementary_move_details(&mut self, _: (usize, usize), retainer_merger_info: Option<<Self::RetainerManager as RetainerManager<Self::ElementType>>::RetainerMergerInfoType>);
    fn board_update_after_move(&mut self, _: AllowedMoves);
    fn board_game_status_fn(&self) -> GameStatus;


}


impl<GameVariant> Swap2DGame<GameVariant>
    where Swap2DGame<GameVariant>:Swap2DGameConfig,
{
        pub fn game_init(board_size: (usize, usize), board_capacity: (usize, usize), game_variant_data: GameVariant) -> Self {
        Self {
            board_top_left_corner: (0, 0),
            board_size,
            board_capacity,
            game_status: GameStatus::INPROGRESS,
            game_variant_data,
        }
    }

    fn game_end(&mut self) {
        // to be implemented if needed
    }

    pub fn step_2d(&self, idx: (usize, usize), move_type:AllowedMoves) -> BoardIndex {
        match move_type {
            AllowedMoves::DOWN if idx.0 < self.board_top_left_corner.0 + self.board_size.0 - 1 => BoardIndex::CorrectIndex((idx.0 + 1, idx.1)),
            AllowedMoves::UP if idx.0 > self.board_top_left_corner.0 => BoardIndex::CorrectIndex((idx.0 - 1, idx.1)),
            AllowedMoves::LEFT if idx.1 > self.board_top_left_corner.1 => BoardIndex::CorrectIndex((idx.0, idx.1 - 1)),
            AllowedMoves::RIGHT if idx.1 < self.board_top_left_corner.1 + self.board_size.1 - 1 => BoardIndex::CorrectIndex((idx.0 , idx.1 + 1)),
            _ => BoardIndex::OutOfBounds,
        }
    }
    pub fn move_generic(&mut self, move_type: AllowedMoves) {
        match self.game_status {
            GameStatus::INPROGRESS => {}
            GameStatus::END_FAIL | GameStatus::END_SUCCESS => {
                return;
            }
            _ => {}
        }

        self.merge(move_type);
        self.board_update_after_move(move_type);

        let status = self.board_game_status_fn();
        match status {
            GameStatus::INPROGRESS => {}
            GameStatus::END_FAIL | GameStatus::END_SUCCESS => {
                self.game_end();
            }
        }
        self.game_status = status;     
    }

    pub fn merge(&mut self, move_direction: AllowedMoves) {

        let mut start_corner: BoardIndex;
        let mut inner_move : AllowedMoves;
        let mut outer_move : AllowedMoves;
        let mut board_index_outer: BoardIndex;
        let mut board_index_inner: BoardIndex;
        let mut board_index_to_be_filled: BoardIndex;

        match move_direction {
            AllowedMoves::LEFT => {
                start_corner = BoardIndex::CorrectIndex((self.board_top_left_corner.0, self.board_top_left_corner.1));
                inner_move = AllowedMoves::RIGHT;
                outer_move = AllowedMoves::DOWN;
            }
            AllowedMoves::UP => {
                start_corner = BoardIndex::CorrectIndex((self.board_top_left_corner.0 , self.board_top_left_corner.1));
                inner_move = AllowedMoves::DOWN;
                outer_move = AllowedMoves::RIGHT;        }
            AllowedMoves::RIGHT => {
                start_corner = BoardIndex::CorrectIndex((self.board_top_left_corner.0, self.board_top_left_corner.1 + self.board_size.1 -1));
                inner_move = AllowedMoves::LEFT;
                outer_move = AllowedMoves::DOWN;
            }
            AllowedMoves::DOWN => {
                start_corner = BoardIndex::CorrectIndex((self.board_top_left_corner.0 + self.board_size.0 -1, self.board_top_left_corner.1));
                inner_move = AllowedMoves::UP;
                outer_move = AllowedMoves::RIGHT;
            }
        }

        board_index_outer = start_corner;
        board_index_inner = start_corner;

        while let BoardIndex::CorrectIndex(IdxOuter) = board_index_outer {

            let mut retainer = <Swap2DGame<GameVariant> as Swap2DGameConfig>::RetainerManager::new();
            board_index_to_be_filled = board_index_outer;
            board_index_inner = board_index_outer;

            while let BoardIndex::CorrectIndex(IdxInner) = board_index_inner{
                let t= self.board_get_element(IdxInner);
                if let BoardIndex::CorrectIndex(IdxToBeFilled) = board_index_to_be_filled {
                    match retainer.push_and_pop_if_filled(IdxInner, t) {
                        (RetainerManagerElementType::GameElement(a),info) =>{
                            self.board_set_element(IdxToBeFilled, Some(a));
                            self.board_elementary_move_details(IdxToBeFilled, Some(info));
                            
                            board_index_to_be_filled = self.step_2d(IdxToBeFilled, inner_move);
                        }
                        (RetainerManagerElementType::IncrementalIgnore,info) => {
                            self.board_set_element(IdxToBeFilled, None);
                            self.board_elementary_move_details(IdxToBeFilled, None);
                            board_index_to_be_filled = self.step_2d(IdxToBeFilled, inner_move);
                        }
                        (RetainerManagerElementType::NeutralIgnore,_) => {
                        }
                    }
                }
                //println!("{:?}", IdxInner);
                board_index_inner = self.step_2d(IdxInner, inner_move);
            }

            //flush remaining retainer buffer
            while let BoardIndex::CorrectIndex(IdxInner) = board_index_to_be_filled{
                match retainer.pop() {
                    (RetainerManagerElementType::GameElement(a),info)  => {
                        self.board_set_element(IdxInner, Some(a));
                        self.board_elementary_move_details(IdxInner, Some(info));
                    }
                    (RetainerManagerElementType::IncrementalIgnore,info) => {
                        self.board_set_element(IdxInner, None);
                        self.board_elementary_move_details(IdxInner, Some(info));
                    }
                    (RetainerManagerElementType::NeutralIgnore,_) => {
                        self.board_set_element(IdxInner, None);
                        self.board_elementary_move_details(IdxInner, None);
                    }                    
                }
                board_index_to_be_filled = self.step_2d(IdxInner, inner_move);
            }
            board_index_outer =self.step_2d(IdxOuter, outer_move);
        }
    }        


}
