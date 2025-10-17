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

pub struct Container<C: Component> {
    square: CandySquare,
    pub(crate) layout: Layout,
    children: Vec<C>,
    ignore_overflow: bool,
}

impl<C: Component> Component for Container<C> {
    fn render(&self, renderer: &mut crate::ui::component::ComponentRenderer) {
        if self.square.rule.get_color().w != 0.0 && self.square.rule.border_color.w != 0.0 {
            renderer.square(&self.square);
        }
        for child in &self.children {
            child.render(renderer);
        }
    }
    fn resize(&mut self, rect: crate::helpers::rect::Rect) {
        self.square.position_mut().x = rect.x;
        self.square.position_mut().y = rect.y;
        self.square.size_mut().x = rect.width;
        self.square.size_mut().y = rect.height;

        let calc = self.layout.calculate(rect, self.ignore_overflow);

        for (idx, rec) in calc.into_iter().enumerate() {
            self.children[idx].resize(rec);
        }
    }
    fn apply_style(&mut self, style: &dyn Style) {
        self.square.apply_style(style);
    }
    fn position(&self) -> nalgebra::Vector2<f32> {
        *self.square.position()
    }
    fn position_mut(&mut self) -> &mut nalgebra::Vector2<f32> {
        self.square.position_mut()
    }

    fn apply_offset(&mut self, offset: nalgebra::Vector2<f32>) {
        *self.position_mut() += offset;
        for child in &mut self.children {
            child.apply_offset(offset);
        }
    }
}

impl<C: Component> Container<C> {
    pub fn new(layout: Layout, ignore_overflow: bool) -> Self {
        Self {
            ignore_overflow,
            layout,
            square: CandySquare::default(),
            children: Vec::new(),
        }
    }

    ///Adds the given `child` and `def` at the provided `index`. If `index` > `len(children)`, then the provided `child` is inserted as
    ///the last one
    pub fn add_child_at(&mut self, child: C, def: DefinitionRect, index: usize) -> &mut Self {
        if index >= self.children.len() {
            self.add_child(child, def)
        } else {
            self.children.insert(index, child);
            self.layout.boxes.insert(index, def);
            self
        }
    }

    ///Clears all the children this Container has and returns them with their respective layout
    pub fn clear_children(&mut self) -> Vec<(C, DefinitionRect)> {
        let children = std::mem::take(&mut self.children);
        let layouts = std::mem::take(&mut self.layout.boxes);
        debug_assert!(children.len() == layouts.len());
        children
            .into_iter()
            .zip(layouts.into_iter())
            .collect::<Vec<(_, _)>>()
    }

    #[inline]
    ///Appends the given `child` on this container without a definition. Note that if the amount of deffinition don't match, this will lead to bugs
    pub unsafe fn add_child_unsafe(&mut self, child: C) -> &mut Self {
        self.children.push(child);
        self
    }

    #[inline]
    ///Adds the given `child` as the new last one with the given `def` rect for resizing.
    pub fn add_child(&mut self, child: C, def: DefinitionRect) -> &mut Self {
        self.children.push(child);
        self.layout.with_definition(def);
        self
    }

    #[inline]
    ///Removes the child at the provided `index`
    pub fn remove_children_at_index(&mut self, index: usize) -> C {
        self.children.remove(index)
    }

    ///Iterates over the children using `f`, on the first true return, that entity is then removed and returned
    pub fn remove_children_where<F>(&mut self, f: F) -> Option<C>
    where
        F: Fn(&C) -> bool,
    {
        self.children
            .iter()
            .position(f)
            .map(|idx| self.remove_children_at_index(idx))
    }

    ///Retrieves all the children of this Container
    pub fn children(&self) -> &Vec<C> {
        &self.children
    }

    ///Retrieves all the children of this Container
    pub fn children_mut(&mut self) -> &mut Vec<C> {
        &mut self.children
    }
}

impl<C: Component> Deref for Container<C> {
    type Target = CandySquare;
    fn deref(&self) -> &Self::Target {
        &self.square
    }
}

impl<C: Component> DerefMut for Container<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.square
    }
}
