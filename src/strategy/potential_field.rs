use crate::model::{Unit, Vec2};

#[derive(Clone, Debug)]
pub struct PotentialFieldNode {
    pos: Vec2,
    score: f64,
}

#[derive(Clone, Debug)]
pub struct PotentialField {
    nodes: Vec<PotentialFieldNode>,
}

impl PotentialField {
    pub const fn new() -> Self {
        PotentialField {
            nodes: Vec::new()
        }
    }
    pub fn find_best(&self) -> Option<PotentialFieldNode> {
        self.nodes.iter()
            .max_by_key(|e| e.score.ceil() as i64)
            .map(|e| e.clone())
    }
}
