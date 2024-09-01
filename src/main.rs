mod enums;
mod request;
mod structs;
mod components {
    pub mod preview;
    pub mod stories;
}

use components::preview::Preview;
use components::stories::Stories;
use dioxus::hooks::use_context_provider;
use dioxus::prelude::*;
use enums::PreviewState;

fn main() {
    launch(App);
}

pub fn App() -> Element {
    use_context_provider(|| Signal::new(PreviewState::Unset));
    rsx! {
        div { display: "flex", flex_direction: "row", width: "100%",
            div { width: "50%", Stories {} }
            div { width: "50%", Preview {} }
        }
    }
}
