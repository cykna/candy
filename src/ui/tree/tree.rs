use super::super::layout::CandyLayout;
use std::{
    collections::{HashSet, VecDeque},
    ops::{Deref, DerefMut},
};

use nalgebra::{Vector2, Vector4};
use pulse::{
    Pulse,
    flume::{self, unbounded},
};
use slotmap::SlotMap;
use smol_str::SmolStr;
use taffy::{Layout, NodeId, Style};

use crate::{
    helpers::in_bounds_of,
    ui::{
        component::{Component, ComponentRenderer, DummyComponent},
        layout::error::LayoutError,
    },
};

use super::node::CandyKey;

pub type CandyRawTree<M> = SlotMap<CandyKey, Box<dyn Component<M> + 'static>>;
///Tree used to control the elements, as well as giving them a parent/children relation
pub struct CandyTree<M> {
    elements: CandyRawTree<M>,
    roots: Vec<CandyKey>,
    layout: CandyLayout,
    size: Vector2<f32>,
    rx: flume::Receiver<CandyKey>,
    tx: flume::Sender<CandyKey>,
}

impl<M> CandyTree<M> {
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

    pub fn get_element_with_id(&self, key: CandyKey) -> Option<&Box<dyn Component<M>>> {
        self.elements.get(key)
    }

    pub fn get_mut_element_with_id(&mut self, key: CandyKey) -> Option<&mut Box<dyn Component<M>>> {
        self.elements.get_mut(key)
    }

    ///Gets the size of the Tree. This should be the equivalent to the window it's at
    pub fn size(&self) -> Vector2<f32> {
        self.size
    }

    ///Creates a new Pulse where this tree is Owner of
    #[inline]
    pub fn create_signal<T>(&self, data: T) -> Pulse<T, CandyKey> {
        pulse::Pulse::new(data, self.tx.clone())
    }

    ///Returns all the children of the element with the given `key`. None if the element doesn't exist
    pub fn children_of(&self, key: CandyKey) -> Option<Vec<&Box<dyn Component<M>>>> {
        if let Some(element) = self.elements.get(key) {
            let mut out = Vec::new();
            for child in element.children() {
                out.push(&self.elements[child]);
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
        element: &Box<dyn Component<M>>,
        set: &mut HashSet<CandyKey>,
        key: &CandyKey,
    ) {
        if set.contains(key) {
            return;
        }
        set.insert(*key);
        let content = element.render(self);
        content.render(painter);
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

    pub fn append_component<C: Component<M> + 'static>(&mut self, parent: Option<CandyKey>) -> () {
        //let element = self.elements.insert(Box::new(DummyComponent));
        //self.elements[element] = Box::new(C::new(self, parent, element));
        //element
    }

    ///Appends the given `root` on this ui as a 'root' element and returns it's ID
    pub fn append_element(&mut self, element: Box<dyn Component<M>>) -> CandyKey {
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

    ///Resizes the element with the given key
    #[inline]
    pub fn resize_element_with_key(&mut self, key: CandyKey) {
        let Some(_) = self.elements.get_mut(key) else {
            return;
        };
        //element.resize(self.layout.layout_of(element.layout()).unwrap());
    }

    pub fn resize_all(&mut self) {
        for element in self.roots.iter() {
            let Some(_) = self.elements.get_mut(*element) else {
                continue;
            };
        }
    }

    ///Tests if the `position` is inside the bounds of some child of `element` if not, check for `element`
    ///This function is recursive and used to follow z-index rules.
    ///Returns the Key of the deepest element that reaches so, if none matches, returns None
    pub fn find_deepest_at<'a>(
        &'a self,
        element: &'a Box<dyn Component<M>>,
        position: Vector2<f32>,
    ) -> Option<&'a Box<dyn Component<M>>> {
        for child in element.children() {
            if let Some(e) = self.find_deepest_at(self.elements.get(child).unwrap(), position) {
                return Some(e);
            };
        }
        let bounds = self.layout_of(element.layout()).unwrap();
        let bounds = Vector4::new(
            bounds.location.x,
            bounds.location.y,
            bounds.location.x + bounds.size.width,
            bounds.location.y,
        );
        if in_bounds_of(bounds, position) {
            Some(element)
        } else {
            None
        }
    }

    ///Tries to get the key of the element which `position` is in bounds of. This will check for children first, if none, then it will check for the children parent.
    pub fn get_element_at(&self, position: Vector2<f32>) -> Option<&Box<dyn Component<M>>> {
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

    ///Appends the style with the given `name` with the given `parent_layout` and returns the id
    pub fn use_style(&mut self, name: SmolStr, parent_layout: Option<NodeId>) -> NodeId {
        self.layout
            .create_element_style(parent_layout, name)
            .unwrap()
    }

    ///Gets the Layout Id of the element with the given `key`.
    pub fn style_id_of(&self, key: Option<CandyKey>) -> Option<NodeId> {
        let Some(key) = key else {
            return Some(self.layout.root());
        };
        self.elements.get(key).map(|c| c.layout())
    }

    pub fn layout_of(&self, id: NodeId) -> Result<&Layout, LayoutError> {
        self.layout.layout_of(id)
    }
}

impl<M> Deref for CandyTree<M> {
    type Target = CandyLayout;
    fn deref(&self) -> &Self::Target {
        &self.layout
    }
}

impl<M> DerefMut for CandyTree<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.layout
    }
}
