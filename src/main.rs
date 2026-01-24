

mod game;
use std::f64::consts::{PI, FRAC_PI_2,FRAC_PI_8, TAU};

use crate::game::game_variants::game_variant_0::{GameVariant0, AllowedMoves, GameStatus};

use dioxus::{html::g::direction, prelude::*};


const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const COMPONENTS_CSS: Asset = asset!("/assets/dx-components-theme.css");
const SCRIPT_JS: Asset = asset!("/assets/script.js");




#[derive(Routable, Clone, Copy)]
#[rustfmt::skip]
enum Route {
    /// The home page that present the game options
    #[route("/")]
    Home {},
    
    #[route("/classic_2048")]
    Classic2048 {},

    #[route("/flexibleGame")]
    FlexibleGame {},
    
}

// For each direction we have a fixed angle and for each direction we have an angle of
// sensitivity where if angle detected is between fixed_angle -+ angle_sensitivity, so 
// we conclude that it is a part of this direction
//
// arguments : 
// min_sensitivity : it is the minimum length of swap path to be accepted
// angle_sensitivity : to conclude if it's part of certain direction
// diff_x : is swap path difference between last point x coordinates and first x point
// diff_y : is swap path difference between last point y coordinates and first y point
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

const SENSITIVITY : f64 = 80.0;
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


fn main() {
    //dioxus::launch(App);
    dioxus::launch(|| rsx! {
        Router::<Route> {}
    });
}

#[component]
fn Head() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: COMPONENTS_CSS }
    }
}

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { class: "fixed top-0 left-0 w-full bg-[#8f7a66] shadow-md z-50",
            div { class: "max-w-6xl mx-auto px-4",
                div { class: "flex justify-between items-center h-16",

                    // Logo
                    Link {
                        to: Route::Home {},
                        class: "text-white text-xl font-bold",
                        "2048"
                    }

                    // Links
                    div { class: "flex gap-6",
                        Link {
                            to: Route::Home {},
                            class: "text-white hover:text-[#f2e8dc]",
                            "Home"
                        }
                                        // Link {
                    //     to: Route::Classic2048 {},
                    //     class: "text-white hover:text-[#f2e8dc]",
                    //     "Classic 2048"
                    // }
                    // Link {
                    //     to: Route::FlexibleGame {},
                    //     class: "text-white hover:text-[#f2e8dc]",
                    //     "Flexible Game"
                    // }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Home() -> Element {
    rsx!{
        Head {}
        div { class: "bg-[#faf8ef]",
            Navbar {}
            div { class: "pt-20 max-w-6xl mx-auto px-6 min-h-[100dvh]",

                // DESCRIPTION
                section { class: "text-center mb-12",

                    h1 { class: "text-4xl font-bold text-[#776e65] mb-4", "Welcome to 2048" }

                    p { class: "text-lg text-[#776e65] max-w-2xl mx-auto",
                        "Choose a variant of the classic 2048 puzzle game. "
                        "Each mode offers a unique twist on the original gameplay."
                    }
                }

                // VARIANTS GRID
                section { class: "grid gap-8 sm:grid-cols-2 lg:grid-cols-3",

                    // Variant Card — Classic
                    Link {
                        to: Route::Classic2048 {},
                        class: "bg-white rounded-xl shadow hover:shadow-lg transition p-4 text-center",

                        h2 { class: "text-xl font-semibold text-[#776e65] mt-2", "Classic 2048" }

                        p { class: "text-sm text-[#776e65] mt-2",
                            "The original 4×4 grid. Combine tiles to reach 2048!"
                        }
                    }

                    // Placeholder Variant
                    div { class: "bg-white rounded-xl shadow p-4 text-center opacity-60 cursor-not-allowed",

                        h2 { class: "text-xl font-semibold text-[#776e65] mt-2", "More Coming Soon" }

                        p { class: "text-sm text-[#776e65] mt-2",
                            "New modes and ideas are on the way."
                        }
                    }
                
                }
            }
        
        }
    }
}


#[component]
pub fn Classic2048() -> Element {
    rsx!{
        Head {}
        div { class: "bg-[#faf8ef]",
            Navbar {}
            Classic2048Inner {}
        }
    }
}

#[component]
pub fn FlexibleGame() -> Element {
    rsx!{
        Head {}
        div { class: "bg-[#faf8ef] min-h-[100dvh] pt-20",
            Navbar {}
            div { class: "text-2xl text-black", "FlexibleGame - To be implemented" }
        }
    }
}

#[component]
pub fn Classic2048Inner() -> Element {
    

    let mut touch_start_event: Signal<Option<TouchEvent>> = use_signal(|| None);
    
    let mut logs = use_signal::<Vec<String>>(Vec::new);

    const c_width : usize = 4;
    const c_height : usize = 4;
    let mut g = GameVariant0::<c_width, c_height>::new_game();
    let mut g_signal = use_signal(move || g);

    fn get_class_x(el: Option<i32>) -> &'static str {
        match el {
            None => "aspect-square flex items-center justify-center w-[5ch] font-bold bg-[#cdc1b4] rounded text-2xl",

            Some(2) => "aspect-square bg-[#cdc1b4] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-[#776e65]",
            Some(4) => "aspect-square bg-[#eee4da] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-[#776e65]",
            Some(8) => "aspect-square bg-[#ede0c8] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-[#776e65]",
            Some(16) => "aspect-square bg-[#f2b179] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-white",
            Some(32) => "aspect-square bg-[#f59563] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-white",
            Some(64) => "aspect-square bg-[#f67c5f] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-white",
            Some(128) => "aspect-square bg-[#f65e3b] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-white",
            Some(256) => "aspect-square bg-[#edcf72] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-white",
            Some(512) => "aspect-square bg-[#edcc61] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-white",
            Some(1024) => "aspect-square bg-[#edc850] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-white",
            Some(2048) => "aspect-square bg-[#edc22e] rounded flex items-center justify-center text-2xl w-[5ch] font-bold text-white",
            _ => "aspect-square bg-black rounded",
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
            h1 { class: "absolute top-20 text-4xl font-bold text-[#776e65]", "Classic 2048" }
            div { class: "wrapper",
                div { class: "mb-4 flex justify-between",

                    button {
                        class: "px-4 py-2 bg-[#8f7a66] text-white rounded hover:bg-[#7c6957]",
                        onclick: move |_| {
                            *g_signal.write() = GameVariant0::<c_width, c_height>::new_game();
                        },
                        "Restart"
                    }
                    div {
                        "Score : "
                        {g_signal.read().game_variant_data.score.to_string()}
                    }
                
                }

                div { class: "bg-[#bbada0] p-4 rounded-lg shadow-lg",
                    div { id: "board", class: "grid grid-cols-{c_width} gap-3",

                        for i in 0..c_height {
                            for j in 0..c_width {
                                div { class: get_class_x(g_signal.read().game_variant_data.array[i][j]),
                                    {
                                        g_signal
                                            .read()
                                            .game_variant_data
                                            .array[i][j]
                                            .map(|v| v.to_string())
                                            .unwrap_or_default()
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
                                *g_signal.write() = GameVariant0::<c_width, c_height>::new_game();
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
                                    *g_signal.write() = GameVariant0::<c_width, c_height>::new_game();
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


