pub struct Scene {
    pub nodes: Vec<Node>,
}

impl Scene {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn evaluate(&mut self, _t: f32) {
        // TODO: evaluate transforms per frame
    }
}

pub enum Node {
    Text(TextNode),
    Image(ImageNode),
}

pub struct TextNode {
    pub content: String,
}

pub struct ImageNode {
    pub path: String,
}