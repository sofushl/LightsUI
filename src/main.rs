mod app;
mod components;

use app::App;
use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}
