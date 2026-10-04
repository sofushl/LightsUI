use crate::components::display_t;
use crate::triangle;

use topcoat::{
    Result,
    context::Cx,
    router::{error::RouterErrorExt, page, path_param},
    view::{View, view},
};

#[page("/")]
pub async fn home() -> Result<impl View> {
    let triangles = triangle::all();

    Ok(view! {
        <h1>"Triangles"</h1>
        for t in triangles {
            display_t(
                id: &t.id,
                level: &t.level,
                colour: t.colour.as_str(),
                mode: t.mode.as_str()
            )
        }
    })
}

#[page("/triangles")]
pub async fn about() -> Result<impl View> {
    Ok(view! { <h1>"Triangles overview"</h1> })
}

path_param!(triangle_id: u8, error=not_found);
#[page("/triangles/{triangle_id}")]
pub async fn triabgle_detail(cx: &Cx) -> Result<impl View> {
    let id: &u8 = path_param::<TriangleId>(cx)?;
    let t = triangle::find(*id).ok_or_not_found()?;

    Ok(view! {
        <div>
            display_t(
                id: id,
                level: &t.level,
                colour: t.colour.as_str(),
                mode: t.mode.as_str()
            )
        </div>
    })
}
