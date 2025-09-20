use std::ops::{Deref, DerefMut};

use crate::{
    elements::CandySquare,
    renderer::twod::BiDimensionalPainter,
    ui::{
        component::Component,
        styling::{
            layout::{DefinitionRect, Layout},
            style::Style,
        },
    },
};

pub struct Container {
    square: CandySquare,
    layout: Layout,
    children: Vec<Box<dyn Component>>,
}

impl Component for Container {
    fn render(&self, renderer: &mut crate::ui::component::ComponentRenderer) {
        if self.square.rule.get_color().w == 0.0 && self.square.rule.border_color.w == 0.0 {
            return;
        }
        renderer.square(&self.square);
    }
    fn resize(&mut self, rect: crate::helpers::rect::Rect) {
        self.square.position_mut().x = rect.x;
        self.square.position_mut().y = rect.y;
        self.square.size_mut().x = rect.width;
        self.square.size_mut().y = rect.height;
        let calc = self.layout.calculate(rect);
        for (idx, rec) in calc.into_iter().enumerate() {
            self.children[idx].resize(rec);
        }
    }
    fn apply_style(&mut self, style: &dyn Style) {
        self.square.apply_style(style);
    }
}

impl Container {
    pub fn new(layout: Layout) -> Self {
        Self {
            layout,
            square: CandySquare::default(),
            children: Vec::new(),
        }
    }

    ///Adds the given `child` and `def` at the provided `index`. If `index` > `len(children)`, then the provided `child` is inserted as
    ///the last one
    pub fn add_child_at<C: Component + 'static>(
        &mut self,
        child: C,
        def: DefinitionRect,
        index: usize,
    ) -> &mut Self {
        if index >= self.children.len() {
            self.add_child(child, def)
        } else {
            self.children.insert(index, Box::new(child));
            self.layout.boxes.insert(index, def);
            self
        }
    }

    ///Clears all the children this Container has and returns them with their respective layout
    pub fn clear_children(&mut self) -> Vec<(Box<dyn Component>, DefinitionRect)> {
        let children = std::mem::take(&mut self.children);
        let layouts = std::mem::take(&mut self.layout.boxes);
        debug_assert!(children.len() == layouts.len());
        children
            .into_iter()
            .zip(layouts.into_iter())
            .collect::<Vec<(_, _)>>()
    }

    #[inline]
    ///Adds the given `child` as the new last one with the given `def` rect for resizing.
    pub fn add_child<C>(&mut self, child: C, def: DefinitionRect) -> &mut Self
    where
        C: Component + 'static,
    {
        self.children.push(Box::new(child));
        self.layout.with_definition(def);
        self
    }

    #[inline]
    ///Removes the child at the provided `index`
    pub fn remove_children_at_index(&mut self, index: usize) -> Box<dyn Component> {
        self.children.remove(index)
    }

    ///Iterates over the children using `f`, on the first true return, that entity is then removed and returned
    pub fn remove_children_where<F>(&mut self, f: F) -> Option<Box<dyn Component>>
    where
        F: Fn(&Box<dyn Component>) -> bool,
    {
        self.children
            .iter()
            .position(f)
            .map(|idx| self.remove_children_at_index(idx))
    }

    ///Retrieves all the children of this Container
    pub fn children(&self) -> &[Box<dyn Component>] {
        &self.children
    }
}

impl Deref for Container {
    type Target = CandySquare;
    fn deref(&self) -> &Self::Target {
        &self.square
    }
}

impl DerefMut for Container {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.square
    }
}
