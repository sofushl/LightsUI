#[derive(Clone, serde::Serialize)]
pub struct Triangle {
    pub id: u8,
    pub level: u8,
    pub colour: String,
    pub mode: String,
}

pub fn all() -> Vec<Triangle> {
    vec![
        Triangle {
            id: 1,
            level: 255,
            colour: "red".to_string(),
            mode: "colour".to_string(),
        },
        Triangle {
            id: 2,
            level: 255,
            colour: "green".to_string(),
            mode: "colour".to_string(),
        },
        Triangle {
            id: 3,
            level: 255,
            colour: "blue".to_string(),
            mode: "colour".to_string(),
        },
    ]
}

pub fn find(id: u8) -> Option<Triangle> {
    all()
        .into_iter()
        .find(|triangle: &Triangle| triangle.id == id)
}
