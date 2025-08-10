use std::collections::HashSet;

use slotmap::SlotMap;

use crate::renderer::twod::BiDimensionalPainter;

use super::node::{CandyKey, CandyNode, ElementBuilder};

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

    ///Renders the given `element` using the `painter` and it's children. `set` is used to get track of which
    ///elements were already drawn, and `key` is the key of the `element` is going to be drawed
    pub fn render_element(
        &self,
        painter: &mut P,
        element: &CandyNode<P>,
        set: &mut HashSet<CandyKey>,
        key: &CandyKey,
    ) {
        if set.contains(key) {
            return;
        }
        set.insert(*key);
        element.render(painter);
        for child_key in element.children() {
            if let Some(child) = self.elements.get(*child_key) {
                self.render_element(painter, child, set, child_key);
            }
        }
    }

    ///Render all the tree using the given `painter`
    pub fn render(&self, painter: &mut P) {
        let mut set = HashSet::new();
        for (key, el) in self.elements.iter() {
            if set.contains(&key) {
                continue;
            }
            self.render_element(painter, el, &mut set, &key);
        }
    }

    ///Removes the element with the given `element` key and it's children recursively
    pub fn remove_element(&mut self, element: CandyKey) {
        if let Some(element) = self.elements.remove(element) {
            for child in element.children() {
                self.remove_element(*child);
            }
        }
    }

    ///Appends the given `root` on this ui as a 'root' element and returns it's ID
    pub fn append_root(&mut self, element: ElementBuilder<P>) -> CandyKey {
        let mut children = Vec::new();
        for child in element.children {
            let child_key = self.append_root(child);
            children.push(child_key);
        }
        let mut node = CandyNode::new(element.inner);
        node.add_children(children);
        self.elements.insert(node)
    }

    #[inline]
    ///Clears all the elements on this UI, thus theyre removed
    pub fn clear(&mut self) {
        self.elements.clear();
    }
}
