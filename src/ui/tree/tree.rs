use super::super::layout::CandyLayout;
use std::collections::HashSet;

use nalgebra::Vector2;
use pulse::{
    Pulse,
    flume::{self, unbounded},
};
use slotmap::SlotMap;
use taffy::{NodeId, Style};

use crate::{
    elements::CandyElement,
    helpers::in_bounds_of,
    ui::component::{Component, ComponentRenderer},
};

use super::node::{CandyKey, CandyNode, ElementBuilder};

pub type CandyRawTree = SlotMap<CandyKey, Box<dyn Component + 'static>>;
///Tree used to control the elements, as well as giving them a parent/children relation
pub struct CandyTree {
    elements: CandyRawTree,
    layout: CandyLayout,
    size: Vector2<f32>,
    rx: flume::Receiver<CandyKey>,
    tx: flume::Sender<CandyKey>,
}

impl CandyTree {
    pub fn new(width: f32, height: f32) -> Self {
        let (tx, rx) = unbounded();
        let mut s = Self {
            layout: CandyLayout::new(),
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
    pub fn children_of(&self, key: CandyKey) -> Option<Vec<&Box<dyn Component>>> {
        if let Some(element) = self.elements.get(key) {
            let mut out = Vec::new();
            for child in element.children() {
                out.push(child);
            }
            Some(out)
        } else {
            None
        }
    }

    ///Renders the given `element` using the `painter` and it's children.
    fn render_element(
        &self,
        painter: &mut ComponentRenderer,
        element: &Box<dyn Component>,
        set: &mut HashSet<CandyKey>,
        key: &CandyKey,
    ) {
        if set.contains(key) {
            return;
        }
        set.insert(*key);
        element.render(painter);
    }

    ///Render all the tree using the given `painter`
    pub fn render(&self, painter: &mut ComponentRenderer) {
        let mut set = HashSet::new();
        for (key, el) in self.elements.iter() {
            if set.contains(&key) {
                continue;
            }
            self.render_element(painter, el, &mut set, &key);
        }
    }

    pub fn create_node(
        &mut self,
        parent_layout: Option<NodeId>,
        builder: ElementBuilder,
    ) -> CandyNode<ComponentRenderer> {
        let style = self
            .layout
            .create_element_style(parent_layout, builder.style_name.unwrap_or_default())
            .unwrap();
        let node = CandyNode::new(builder.inner, style);
        node
    }

    pub fn append_component<C: Component + 'static>(&mut self) -> CandyKey {
        let component = Box::new(C::new(self));
        self.append_element(component)
    }

    ///Appends the given `root` on this ui as a 'root' element and returns it's ID
    pub fn append_element(&mut self, element: Box<dyn Component>) -> CandyKey {
        let out = self.elements.insert(element);
        self.resize_element_with_key(out);
        self.resize(self.size.x, self.size.y);
        out
    }

    #[inline]
    ///Clears all the elements on this UI, thus theyre removed
    pub fn clear(&mut self) {
        self.elements.clear();
    }

    pub fn resize_element_with_key(&mut self, key: CandyKey) {
        let Some(element) = self.elements.get_mut(key) else {
            return;
        };
        element.resize(&self.layout);
    }

    pub fn resize_all(&mut self) {
        for element in self.elements.iter_mut() {
            element.1.resize(&self.layout);
        }
    }

    ///Tests if the `position` is inside the bounds of some child of `element` if not, check for `element`
    ///This function is recursive and used to follow z-index rules.
    ///Returns the Key of the deepest element that reaches so, if none matches, returns None
    pub fn find_deepest_at<'a>(
        &'a self,
        element: &'a Box<dyn Component>,
        position: Vector2<f32>,
    ) -> Option<&'a Box<dyn Component>> {
        for child in element.children() {
            if let Some(e) = self.find_deepest_at(child, position) {
                return Some(e);
            };
        }
        let mut bounds = element.bounds();
        //Resolve due to offset
        if let CandyElement::Text(_) = &element.inner().inner {
            bounds.y -= bounds.w;
        }
        if in_bounds_of(bounds, position) {
            Some(element)
        } else {
            None
        }
    }

    ///Tries to get the key of the element which `position` is in bounds of. This will check for children first, if none, then it will check for the children parent.
    pub fn get_element_at(&self, position: Vector2<f32>) -> Option<&Box<dyn Component>> {
        for (_, el) in self.elements.iter() {
            if let Some(pos) = self.find_deepest_at(el, position) {
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
        self.resize_all();
    }

    pub fn create_style(&mut self, name: &str, style: Style) {
        self.layout.create_style(name.into(), style);
    }
}
