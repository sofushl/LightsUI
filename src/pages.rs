use crate::components::{number_input, string_input, submit_button};

use topcoat::{
    Result,
    context::Cx,
    router::page,
    runtime::signal,
    view::{View, view},
};

#[page("/")]
pub async fn home(cx: &Cx) -> Result<impl View> {
    let mode = signal(cx, || String::from("CHASE"));
    let red = signal(cx, || String::from("255"));
    let green = signal(cx, || String::from("0"));
    let blue = signal(cx, || String::from("0"));

    Ok(view! {
        <h1 class="text-xl font-bold pb-10 underline">"Triangles"</h1>

        string_input(display: "MODE", value: &mode)

        number_input(display: "RED", value: &red)
        number_input(display: "GREEN", value: &green)
        number_input(display: "BLUE", value: &blue)

        submit_button(display: "Submit", mode: &mode, r: &red, g: &green, b: &blue)
    })
}
