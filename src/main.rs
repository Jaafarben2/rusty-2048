
mod frontendcomponents;
mod game;

use dioxus::prelude::*;

use self::frontendcomponents::{classic_2048, variant_1_2048, variant_2_2048, variant_3_2048};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const COMPONENTS_CSS: Asset = asset!("/assets/dx-components-theme.css");

#[derive(Routable, Clone, Copy)]
#[rustfmt::skip]
enum Route {
    /// The home page that present the game options
    #[route("/")]
    Home {},
    
    #[route("/classic_2048")]
    Classic2048 {},

    #[route("/classic_2048_variant_1")]
    Classic2048Variant1 {},

    #[route("/classic_2048_variant_2")]
    Classic2048Variant2 {},

    #[route("/classic_2048_variant_3")]
    Classic2048Variant3 {},
    
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
                    Link {
                        to: Route::Home {},
                        class: "text-white text-xl font-bold",
                        "2048"
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

                    // Variant Card — Classic
                    Link {
                        to: Route::Classic2048Variant1 {},
                        class: "bg-white rounded-xl shadow hover:shadow-lg transition p-4 text-center",

                        h2 { class: "text-xl font-semibold text-[#776e65] mt-2",
                            "Classic 2048 Variant 1"
                        }

                        p { class: "text-sm text-[#776e65] mt-2",
                            "The original 4×4 grid variant 1 with moving blocks. Combine tiles to reach 2048!"
                        }
                    }

                    // Variant Card — Classic
                    Link {
                        to: Route::Classic2048Variant2 {},
                        class: "bg-white rounded-xl shadow hover:shadow-lg transition p-4 text-center",

                        h2 { class: "text-xl font-semibold text-[#776e65] mt-2",
                            "Classic 2048 Variant 2"
                        }

                        p { class: "text-sm text-[#776e65] mt-2",
                            "The original 4×4 grid variant 2 with moving/fixed blocks . Combine tiles to reach 2048!"
                        }
                    }

                    // // Variant Card — Classic
                    // Link {
                    //     to: Route::Classic2048Variant3 {},
                    //     class: "bg-white rounded-xl shadow hover:shadow-lg transition p-4 text-center",

                    //     h2 { class: "text-xl font-semibold text-[#776e65] mt-2",
                    //         "Classic 2048 Variant 3"
                    //     }

                    //     p { class: "text-sm text-[#776e65] mt-2",
                    //         "The original 4×4 grid variant 3 with moving/fixed blocks and flexible size. Combine tiles to reach 2048!"
                    //     }
                    // }

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
            classic_2048 {}
        }
    }
}


#[component]
pub fn Classic2048Variant1() -> Element {
    rsx!{
        Head {}
        div { class: "bg-[#faf8ef]",
            Navbar {}
            variant_1_2048 {}
        }
    }
}


#[component]
pub fn Classic2048Variant2() -> Element {
    rsx!{
        Head {}
        div { class: "bg-[#faf8ef]",
            Navbar {}
            variant_2_2048 {}
        }
    }
}

#[component]
pub fn Classic2048Variant3() -> Element {
    rsx!{
        Head {}
        div { class: "bg-[#faf8ef]",
            Navbar {}
            variant_3_2048 {}
        }
    }
}
