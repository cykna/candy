use slotmap::{DefaultKey, SlotMap};

pub struct Node {
    children: Vec<DefaultKey>,
}

pub struct Tree {
    elements: SlotMap<DefaultKey, Node>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            elements: SlotMap::new(),
        }
    }
    pub fn children_of(&self, element: DefaultKey) -> Vec<&Node> {
        let mut out = Vec::new();
        if let Some(element) = self.elements.get(element) {
            for child in element.children {
                out.push(&self.elements[child]);
            }
        }
        out
    }
}
