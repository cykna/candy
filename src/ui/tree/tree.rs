use slotmap::SlotMap;

use crate::{elements::CandyElement, renderer::twod::BiDimensionalPainter};

use super::node::{CandyKey, CandyNode};

pub type CandyRawTree<P> = SlotMap<CandyKey, CandyNode<P>>;
///Tree used to control the elements, as well as giving them a parent/children relation
pub struct CandyTree<P: BiDimensionalPainter> {
    elements: CandyRawTree<P>,
}

impl<P> CandyTree<P>
where
    P: BiDimensionalPainter,
{
    pub fn new() -> Self {
        Self {
            elements: SlotMap::with_key(),
        }
    }
    ///Returns all the children of the element with the given `key`. None if the element doesn't exist
    pub fn children_of(&self, key: CandyKey) -> Option<Vec<&CandyNode<P>>> {
        if let Some(element) = self.elements.get(key) {
            let mut out = Vec::new();
            for child in element.children() {
                out.push(&self.elements[*child]);
            }
            Some(out)
        } else {
            None
        }
    }

    ///Renders this ui with the given `painter`
    pub fn render_with(&self, painter: &mut P) {
        for (_, child) in self.elements.iter() {
            child.render(painter);
        }
    }

    ///Appends the given `root` on this ui as a 'root' element and returns it's ID
    pub fn append_root(&mut self, root: CandyElement<P>) -> CandyKey {
        self.elements.insert(CandyNode::new(root))
    }
}
