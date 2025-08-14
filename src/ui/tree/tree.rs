use super::super::layout::CandyLayout;
use std::collections::HashSet;

use nalgebra::Vector2;
use pulse::{Pulse, flume};
use slotmap::SlotMap;
use taffy::Style;

use crate::{elements::CandyElement, helpers::in_bounds_of, renderer::twod::BiDimensionalPainter};

use super::node::{CandyKey, CandyNode, ElementBuilder};

pub type CandyRawTree<P> = SlotMap<CandyKey, CandyNode<P>>;
///Tree used to control the elements, as well as giving them a parent/children relation
pub struct CandyTree<P: BiDimensionalPainter> {
    elements: CandyRawTree<P>,
    roots: Vec<CandyKey>,
    layout: CandyLayout,
    size: Vector2<f32>,
    rx: flume::Receiver<CandyKey>,
    tx: flume::Sender<CandyKey>,
}

impl<P> CandyTree<P>
where
    P: BiDimensionalPainter,
{
    pub fn new(width: f32, height: f32) -> Self {
        let (tx, rx) = unbounded();
        let mut s = Self {
            layout: CandyLayout::new(),
            roots: Vec::new(),
            elements: SlotMap::with_key(),
            size: Vector2::new(width, height),
            rx,
            tx,
        };
        s.resize(width, height);
        s
    }

    ///Creates a new Pulse where this tree is Owner of
    #[inline]
    pub fn create_signal<T>(&self, data: T) -> Pulse<T, CandyKey> {
        pulse::Pulse::new(data, self.tx.clone())
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
    fn render_element(
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

    ///Removes the element with the given `element_key` from the tree. Removes it as well from the parent if it's got some, else, tries to remove it from the root
    ///Does the same for it's children recursively
    pub fn remove_element(&mut self, element_key: CandyKey) {
        let Some(element) = self.elements.remove(element_key) else {
            return;
        };

        if let Some(parent_id) = element.parent() {
            let _ = self
                .elements
                .get_mut(parent_id)
                .map(|parent| parent.remove_child(element_key));
        } else if let Some(found) = self.roots.iter().position(|r| *r == element_key) {
            self.roots.swap_remove(found);
        }

        for child in element.children() {
            self.remove_element(*child);
        }
    }

    ///Appends the given element to the root
    #[inline]
    pub fn append_root(&mut self, element: ElementBuilder<P>) -> CandyKey {
        self.append_element(element, None)
    }

    ///Appends the given `root` on this ui as a 'root' element and returns it's ID
    pub fn append_element(
        &mut self,
        element: ElementBuilder<P>,
        parent: Option<CandyKey>,
    ) -> CandyKey {
        let node = CandyNode::new(element.inner, {
            let parent = parent.map(|parent_key| self.elements.get(parent_key).unwrap().style());
            if let Some(style_name) = element.style_name {
                self.layout
                    .create_element_style(parent, style_name)
                    .unwrap()
            } else {
                self.layout
                    .create_raw_style(parent, element.styled)
                    .unwrap()
            }
        });
        let out = self.elements.insert(node);
        if let None = parent {
            self.roots.push(out);
        }
        let mut children = Vec::new();
        for child in element.children {
            let child_key = self.append_element(child, Some(out));
            children.push(child_key);
        }
        self.elements.get_mut(out).unwrap().add_children(children);
        self.resize_element(out);
        out
    }

    #[inline]
    ///Clears all the elements on this UI, thus theyre removed
    pub fn clear(&mut self) {
        self.elements.clear();
        self.roots.clear();
    }

    ///Resizes the element owner of the given `key` and it's children. `processed_key` is used to get track
    pub fn resize_element(&mut self, key: CandyKey) {
        let mut keys = vec![key];
        while let Some(key) = keys.pop() {
            let Some(element) = self.elements.get_mut(key) else {
                continue;
            };
            for child in element.children() {
                keys.push(*child);
            }

            let layout = self.layout.layout_of(element.style()).unwrap();
            element.resize(layout);
        }
    }

    ///Tests if the `position` is inside the bounds of some child of `element` if not, check for `element`
    ///This function is recursive and used to follow z-index rules.
    ///Returns the Key of the deepest element that reaches so, if none matches, returns None
    pub fn find_deepest_at(
        &self,
        element_key: CandyKey,
        element: &CandyNode<P>,
        position: Vector2<f32>,
    ) -> Option<CandyKey> {
        for child in element.children() {
            if let Some(_) =
                self.find_deepest_at(*child, self.elements.get(*child).unwrap(), position)
            {
                return Some(*child);
            };
        }
        let mut bounds = element.bounds();
        //Resolve due to offset
        if let CandyElement::Text(_) = &element.inner {
            bounds.y -= bounds.w;
        }
        if in_bounds_of(bounds, position) {
            Some(element_key)
        } else {
            None
        }
    }

    ///Tries to get the key of the element which `position` is in bounds of. This will check for children first, if none, then it will check for the children parent.
    pub fn get_element_at(&self, position: Vector2<f32>) -> Option<CandyKey> {
        for root in self.roots.iter() {
            if let Some(pos) =
                self.find_deepest_at(*root, self.elements.get(*root).unwrap(), position)
            {
                return Some(pos);
            }
        }
        None
    }

    ///Resizes the layout of the UI-tree with the given `width` and `height` and recomputes the all the elements
    pub fn resize(&mut self, width: f32, height: f32) {
        self.size.x = width;
        self.size.y = height;
        self.layout.recalculate(width, height).unwrap();
        let mut idx = 0;
        while let Some(root) = self.roots.get(idx) {
            self.resize_element(*root);
            idx += 1;
        }
    }

    pub fn create_style(&mut self, name: &str, style: Style) {
        self.layout.create_style(name.into(), style);
    }
}
