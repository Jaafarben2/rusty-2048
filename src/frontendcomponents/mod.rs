


use dioxus::prelude::*;

use crate::game::game_variants::game_variant_1::{SpecificGame, GameVariant, AllowedMoves, GameStatus, SpecificElementType, get_rand_idx, set_nth_none_element, get_rand_value };

const SCRIPT_JS: Asset = asset!("/assets/script.js");

use std::f64::consts::{PI, FRAC_PI_2,FRAC_PI_8, TAU};



#[derive(Debug, Clone,Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down
}

struct DirectionInfoSwapType {
    asociated_direction : Direction,
    angle_ranges : [Option<(f64,f64)>;2],
}

const DIRECTION_INFOS : [DirectionInfoSwapType;4]= [
    DirectionInfoSwapType{
        asociated_direction : Direction::Right,
        angle_ranges : [Some((0.0,0.0 + FRAC_PI_8)), Some((TAU-FRAC_PI_8,TAU))],
    },
    DirectionInfoSwapType{
        asociated_direction : Direction::Up,
        angle_ranges : [Some((FRAC_PI_2 - FRAC_PI_8,FRAC_PI_2 + FRAC_PI_8)),None],

    },
    DirectionInfoSwapType{
        asociated_direction : Direction::Left,
        angle_ranges : [Some((PI - FRAC_PI_8,PI + FRAC_PI_8)),None],

    },
    DirectionInfoSwapType{
        asociated_direction : Direction::Down,
        angle_ranges : [Some((3.0*FRAC_PI_2 - FRAC_PI_8,3.0*FRAC_PI_2 + FRAC_PI_8)),None],
    },

];

fn is_part_of_this_angle_range(angle:f64, angle_ref_info:&DirectionInfoSwapType)-> bool{
    for angle_range in angle_ref_info.angle_ranges {
        if let Some((min, max)) = angle_range{
            if angle >= min && angle <= max {
                return true
            }
        }
    }
    false
}

const MIN_LENGTH_SWAP : f64 = 80.0;

fn get_swap_directions(diff_x : f64, diff_y:f64) -> Vec<Direction> {

    let mut directions: Vec<Direction> = Vec::new();;
    let swap_length_p_2 = diff_x.powi(2) + diff_y.powi(2);
    if swap_length_p_2 < MIN_LENGTH_SWAP {
        return directions
    }
    
    let  tan_phi = diff_y / diff_x;

    let mut phi = tan_phi.atan();
    if phi < 0.0 {
        phi = TAU + phi;
        if diff_x < 0.0 {
            phi = phi - PI;
        }
    }
    else if phi > 0.0 && diff_x < 0.0 {
        phi = phi + PI

    }



    for direction_info in DIRECTION_INFOS {
        if is_part_of_this_angle_range(phi, &direction_info) {
            directions.push(direction_info.asociated_direction)
        }
    }
    directions

}


#[component]
pub fn rectangular_board<const C_W:usize, const C_H:usize>( game_init : fn((usize,usize)) -> SpecificGame<C_W,C_H>, size_signal : Signal<(usize,usize)>)-> Element {

    let mut touch_start_event: Signal<Option<TouchEvent>> = use_signal(|| None);
    
    let mut logs = use_signal::<Vec<String>>(Vec::new);

    let mut g = game_init((size_signal.read().0,size_signal.read().1));

    let mut g_signal = use_signal(move || g);

    fn get_class_x(el: Option<SpecificElementType>) -> &'static str {
        match el {
            None => "aspect-square flex items-center justify-center w-[5ch] font-bold bg-[#cdc1b4] rounded text-2xl",

            Some(SpecificElementType::Some(2)) => "aspect-square bg-[#cdc1b4] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-[#776e65]",
            Some(SpecificElementType::Some(4)) => "aspect-square bg-[#eee4da] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-[#776e65]",
            Some(SpecificElementType::Some(8)) => "aspect-square bg-[#ede0c8] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-[#776e65]",
            Some(SpecificElementType::Some(16)) => "aspect-square bg-[#f2b179] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-white",
            Some(SpecificElementType::Some(32)) => "aspect-square bg-[#f59563] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-white",
            Some(SpecificElementType::Some(64)) => "aspect-square bg-[#f67c5f] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-white",
            Some(SpecificElementType::Some(128)) => "aspect-square bg-[#f65e3b] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-white",
            Some(SpecificElementType::Some(256)) => "aspect-square bg-[#edcf72] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-white",
            Some(SpecificElementType::Some(512))=> "aspect-square bg-[#edcc61] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-white",
            Some(SpecificElementType::Some(1024)) => "aspect-square bg-[#edc850] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-white",
            Some(SpecificElementType::Some(2048)) => "aspect-square bg-[#edc22e] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-white",
            Some(SpecificElementType::BlockFixed) => "aspect-square rounded grid grid-cols-8 grid-rows-8",
            Some(SpecificElementType::Block) => "aspect-square rounded grid grid-cols-8 grid-rows-8",

            _ => "aspect-square bg-red rounded",
        }
    }

    rsx! {
        document::Script { src: SCRIPT_JS, defer: true }
        div {
            tabindex: "0",
            class: "flex items-center min-h-[100dvh] justify-center items-center",
            id: "game-container",
            style: "touch-action: none;",

            // keyboard
            onkeydown: move |evt| {
                match evt.key() {
                    Key::ArrowLeft => g_signal.write().move_generic(AllowedMoves::LEFT),
                    Key::ArrowRight => g_signal.write().move_generic(AllowedMoves::RIGHT),
                    Key::ArrowUp => g_signal.write().move_generic(AllowedMoves::UP),
                    Key::ArrowDown => g_signal.write().move_generic(AllowedMoves::DOWN),

                    _ => {}
                }
                let status = g_signal.read().game_status;
                //check game status if end to show game over
                if status == GameStatus::END_FAIL {
                    logs.write().push("Game Over".to_string());

                } else if status == GameStatus::END_SUCCESS {
                    logs.write().push("You Win!".to_string());
                }
            },

            ontouchstart: move |evt| {
                let mut t = touch_start_event.write();
                *t = Some(evt);
            },

            ontouchend: move |evt| {
                let t = touch_start_event.read();
                let mut directions: Vec<Direction> = Vec::new();
                if let Some(var) = t.as_ref() {
                    if let Some(touch_start) = var.touches().first() {
                        if let Some(touch_end) = evt.touches_changed().first() {

                            let diff_x = touch_end.client_coordinates().x
                                - touch_start.client_coordinates().x;
                            let diff_y = touch_end.client_coordinates().y
                                - touch_start.client_coordinates().y;
                            // because the direction axis of y  is down
                            directions = get_swap_directions(diff_x, -diff_y);

                            logs.write().push(format!("directions : {directions:#?}"));
                        }
                    }
                }
                if directions.len() == 1 {
                    match directions[0] {
                        Direction::Left => g_signal.write().move_generic(AllowedMoves::LEFT),
                        Direction::Right => g_signal.write().move_generic(AllowedMoves::RIGHT),
                        Direction::Down => g_signal.write().move_generic(AllowedMoves::DOWN),
                        Direction::Up => g_signal.write().move_generic(AllowedMoves::UP),
                    }

                    let status = g_signal.read().game_status;
                    //check game status if end to show game over
                    if status == GameStatus::END_FAIL {
                        logs.write().push("Game Over".to_string());

                    } else if status == GameStatus::END_SUCCESS {
                        logs.write().push("You Win!".to_string());
                    }
                }

            },
            h1 { class: "absolute top-20 text-4xl font-bold text-[#776e65]", "Classic 2048 Variant 1" }
            div { class: "wrapper",
                div { class: "mb-4 flex justify-between",

                    button {
                        class: "px-4 py-2 bg-[#8f7a66] text-white rounded hover:bg-[#7c6957]",
                        onclick: move |_| {
                            *g_signal.write() = game_init((size_signal.read().0, size_signal.read().1));
                        },
                        "Restart"
                    }
                    div {
                        "Score : "
                        {g_signal.read().game_variant_data.score.to_string()}
                        "gf"
                        {size_signal.read().0.to_string()}
                        {size_signal.read().1.to_string()}
                    
                    }
                
                }

                div { class: "bg-[#bbada0] p-4 rounded-lg shadow-lg",
                    div {
                        id: "board",
                        class: "grid grid-cols-{g_signal.read().board_size.0} gap-3",

                        for i in 0..g_signal.read().board_size.1 {
                            for j in 0..g_signal.read().board_size.0 {
                                div { class: get_class_x(g_signal.read().game_variant_data.array[i][j]),
                                    if let Some(SpecificElementType::Some(num)) = g_signal
                                        .read()
                                        .game_variant_data
                                        .array[i][j]
                                    {
                                        {num.to_string()}
                                    } else if let Some(SpecificElementType::BlockFixed) = g_signal
                                        .read()
                                        .game_variant_data
                                        .array[i][j]
                                    {
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#c05a42]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#c05a42]" }

                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#c05a42]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }

                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#c05a42]" }
                                        div { class: "bg-[#c05a42]" }
                                        div { class: "bg-[#c05a42]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#c05a42]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#a44734]" }

                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#c05a42]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#c05a42]" }
                                        div { class: "bg-[#b5523b]" }

                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#c05a42]" }

                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#c05a42]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#c05a42]" }

                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#c05a42]" }

                                        div { class: "bg-[#c05a42]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#a44734]" }
                                    } else if let Some(SpecificElementType::Block) = g_signal.read().game_variant_data.array[i][j] {
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#c05a42]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#c05a42]" }

                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#c05a42]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }

                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#c05a42]" }
                                        div { class: "bg-[#c05a42]" }
                                        div { class: "bg-[#c05a42]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#c05a42]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#a44734]" }

                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#c05a42]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#c05a42]" }
                                        div { class: "bg-[#b5523b]" }

                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#c05a42]" }

                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#c05a42]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#c05a42]" }

                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#a44734]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }

                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                        div { class: "bg-[#b5523b]" }
                                    } else {
                                        div {}
                                    }
                                }
                            }
                        }
                    }
                    // for debug
                    div { class: "hidden",
                        div { class: "mt-4 p-2 border h-32 overflow-auto text-sm bg-black text-green-400",

                            for log in logs.read().iter().rev() {
                                div { "{log}" }
                            }
                        }
                        button {
                            class: "px-4 py-2 bg-[#8f7a66] text-white rounded hover:bg-[#7c6957]",
                            onclick: move |_| {
                                *g_signal.write() = game_init((size_signal.read().0, size_signal.read().1));
                            },
                            "Restart"
                        }
                    }
                    // Dummy div to make tailwind include these classes
                    div { class: "hidden grid-cols-1 grid-cols-2 grid-cols-3 grid-cols-4 grid-cols-5 grid-cols-6 grid-cols-7 grid-cols-8 grid-cols-9 grid-cols-10 grid-cols-11 grid-cols-12" }
                                //div { class: "hidden w-[1ch] w-[2ch] w-[3ch] w-[4ch] w-[5ch] w-[6ch] w-[7ch] w-[8ch] w-[9ch] w-[10ch] w-[11ch] w-[12ch]" }
                }
            }

            // Game END
            if g_signal.read().game_status == GameStatus::END_FAIL
                || g_signal.read().game_status == GameStatus::END_SUCCESS
            {
                div { class: "absolute inset-0 bg-black/40 rounded-lg flex items-center justify-center",
                    div { class: "bg-[#faf8ef] p-6 rounded-lg shadow-xl text-center w-[90%] max-w-xs",
                        if g_signal.read().game_status == GameStatus::END_FAIL {
                            h2 { class: "text-3xl font-bold text-[#776e65] mb-4", "Game Over" }
                            p { class: "text-[#776e65] mb-6", "No more moves left!" }
                        } else if g_signal.read().game_status == GameStatus::END_SUCCESS {
                            h2 { class: "text-3xl font-bold text-[#776e65] mb-4", "You Win!" }
                        }
                        div { class: "flex gap-3 justify-center",
                            button {
                                class: "px-4 py-2 bg-[#8f7a66] text-white rounded hover:bg-[#7c6957]",
                                onclick: move |_| {
                                    *g_signal.write() = game_init((size_signal.read().0, size_signal.read().1));
                                },
                                "Restart"
                            }
                        }
                    }
                }
            }
        }
    }
}


    

#[component]
pub fn classic_2048() -> Element {
    fn game_init<const C_W:usize,const C_H:usize>(size : (usize, usize)) -> SpecificGame<C_W,C_H> {
        let mut g = GameVariant::<C_W,C_H>::new_game();

        let mut insert_idx = get_rand_idx(0, g.game_variant_data.nones_number);
        set_nth_none_element(&mut g, insert_idx,get_rand_value());
        g
    }
    let size_signal: Signal<(usize, usize)> = use_signal(||(4 as usize,4 as usize));
    rsx!{
        rectangular_board { game_init: game_init::<4, 4>, size_signal }
    }
}

#[component]
pub fn variant_1_2048() -> Element {
    fn game_init<const C_W:usize,const C_H:usize>(size : (usize, usize)) -> SpecificGame<C_W,C_H> {
        let mut g = GameVariant::<C_W,C_H>::new_game();

        let mut insert_idx = get_rand_idx(0, g.game_variant_data.nones_number);
        set_nth_none_element(&mut g, insert_idx,get_rand_value());

        insert_idx = get_rand_idx(0, g.game_variant_data.nones_number);
        set_nth_none_element(&mut g, insert_idx,Some(SpecificElementType::Block));
        g
    }
    
    let size_signal: Signal<(usize, usize)> = use_signal(||(4 as usize,4 as usize));
    rsx!{
        rectangular_board { game_init: game_init::<4, 4>, size_signal }
    }
}

#[component]
pub fn variant_2_2048() -> Element {
    fn game_init<const C_W:usize,const C_H:usize>(size : (usize, usize)) -> SpecificGame<C_W,C_H> {
        let mut g = GameVariant::<C_W,C_H>::new_game();

        let mut insert_idx = get_rand_idx(0, g.game_variant_data.nones_number);
        set_nth_none_element(&mut g, insert_idx,get_rand_value());

        insert_idx = get_rand_idx(0, g.game_variant_data.nones_number);
        set_nth_none_element(&mut g, insert_idx,Some(SpecificElementType::BlockFixed));

        insert_idx = get_rand_idx(0, g.game_variant_data.nones_number);
        set_nth_none_element(&mut g, insert_idx,Some(SpecificElementType::Block));

        insert_idx = get_rand_idx(0, g.game_variant_data.nones_number);
        set_nth_none_element(&mut g, insert_idx,Some(SpecificElementType::Block));
        g
    }
    
    let size_signal = use_signal(||(4 as usize,4 as usize));
    rsx!{

        rectangular_board { game_init: game_init::<4, 4>, size_signal }
    }
}


#[component]
pub fn variant_3_2048() -> Element {
    fn game_init<const C_W:usize,const C_H:usize>(size : (usize, usize)) -> SpecificGame<C_W,C_H> {
        let mut g = GameVariant::<C_W,C_H>::new_game_specific_dim(size).unwrap();

        let mut insert_idx = get_rand_idx(0, g.game_variant_data.nones_number);
        set_nth_none_element(&mut g, insert_idx,get_rand_value());

        insert_idx = get_rand_idx(0, g.game_variant_data.nones_number);
        set_nth_none_element(&mut g, insert_idx,Some(SpecificElementType::BlockFixed));

        insert_idx = get_rand_idx(0, g.game_variant_data.nones_number);
        set_nth_none_element(&mut g, insert_idx,Some(SpecificElementType::Block));

        insert_idx = get_rand_idx(0, g.game_variant_data.nones_number);
        set_nth_none_element(&mut g, insert_idx,Some(SpecificElementType::Block));
        g
    }
    
    let mut size_signal = use_signal(||(4 as usize,4 as usize));
    rsx!{
        //input { oninput: move |el| { (*size_signal.write()).0 = el.value().parse::<usize>().unwrap() } }
        //input { oninput: move |el| { (*size_signal.write()).1 = el.value().parse::<usize>().unwrap() } }
        rectangular_board { game_init: game_init::<5, 5>, size_signal }
    }
}