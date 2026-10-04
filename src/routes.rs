use topcoat::{
    Result,
    router::{content::Json, route},
};

use crate::triangle::{Triangle, all};

#[route(GET "/api/health")]
async fn health() -> Result<&'static str> {
    Ok("ok")
}

#[route(GET "/api/triangles")]
async fn get_triangles() -> Result<Json<Vec<Triangle>>> {
    Ok(Json(all()))
}
