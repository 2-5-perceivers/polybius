mod app;

mod components {
    pub mod copy_icon;
    pub mod footer;
    pub mod form;
    pub mod github_logo;
    pub mod input_bits;
    pub mod input_numeric;
    pub mod input_string;
    pub mod list_tile_switch;
    pub mod navbar;
    pub mod password_viewer;
}

mod traits {
    pub mod data_serialization;
}

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
