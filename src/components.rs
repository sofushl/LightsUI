use topcoat::{
    Result,
    view::{View, component, view},
};

#[component]
pub async fn display_t(id: &u8, level: &u8, colour: &str, mode: &str) -> Result<impl View> {
    let bgcolor = match colour {
        "red" => "bg-red-500",
        "blue" => "bg-blue-500",
        "green" => "bg-green-500",
        _ => "bg-gray-500",
    };

    Ok(view! {
        <a href=(format!("/triangles/{id}"))>
            <div class="border border-amber-100 rounded-lg p-5 mb-3 bg-white">
                <p>
                    "Level: "
                    (level)
                </p>
                <p
                    class=(format!(
                        "inline-block mt-3 text-xs px-2 py-1 rounded text-stone-700 {bgcolor}",
                    ))
                >
                    if mode == "colour" {
                        "Colour: "
                        (colour)
                    } else {
                        "Mode: "
                        (mode)
                    }
                </p>
            </div>
        </a>
    })
}
