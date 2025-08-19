use nalgebra::Vector2;

use crate::helpers::rect::Rect;

#[derive(Debug, Clone)]
pub struct DefinitionRect {
    pub x: Size,
    pub y: Size,
    pub width: Size,
    pub height: Size,
}

#[derive(Debug, Default, Clone)]
pub enum Direction {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    Length(f32),
    Percent(f32),
}

impl Default for Size {
    fn default() -> Self {
        Size::Percent(1.0)
    }
}

#[derive(Default, Debug)]
pub enum Corner {
    TopLeft,      //x,y
    TopRight,     //x+w,y
    MiddleLeft,   //x, y+h/2
    MiddleRight,  //x+w, y+h/2
    MiddleTop,    //x+w/2, y
    MiddleBottom, //x+w/2, y + h
    BottomLeft,   //x, y+h
    BottomRight,  //x+w, y+h
    #[default]
    Center,
    Custom(Size, Size),
}

///Used simply to get the metrics while calculing `Layout`
#[derive(Default, Debug)]
struct CalculationMetrics {
    pub(crate) offset_x: f32,
    pub(crate) offset_y: f32,
    pub(crate) largest_x: f32,
    pub(crate) largest_y: f32,
}

#[derive(Default)]
pub struct Layout {
    boxes: Vec<DefinitionRect>,
    gap: Vector2<Size>,
    corner: Corner,
    direction: Direction,
}

impl Layout {
    #[inline]
    pub fn vertical() -> Self {
        Self::new(Direction::Vertical)
    }

    #[inline]
    pub fn horizontal() -> Self {
        Self::new(Direction::Horizontal)
    }

    #[inline]
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
            ..Default::default()
        }
    }

    pub fn with_corner(mut self, corner: Corner) -> Self {
        self.corner = corner;
        self
    }

    pub fn with_definition(mut self, def: DefinitionRect) -> Self {
        self.boxes.push(def);
        self
    }

    pub fn with_gap(mut self, gap: Vector2<Size>) -> Self {
        self.gap = gap;
        self
    }

    fn calc_space_between(
        x: &mut f32,
        y: &mut f32,
        rect: &Rect,
        out: &mut Rect,
        def: DefinitionRect,
        recip: f32,
    ) {
        match def.y {
            Size::Length(defy) => {
                out.y = defy + *y;
                *y += defy
            }
            Size::Percent(defy) => {
                out.y = defy * rect.y + *y;
                *y += defy * rect.y;
            }
        }
        match def.height {
            Size::Length(defh) => {
                out.height = defh * recip * 2.0;
                *y += out.height;
            }
            Size::Percent(defh) => {
                out.height = defh * rect.height * recip * 2.0;
                *y += out.height;
            }
        }

        if *y > rect.right() {
            *y = 0.0;
        }

        match def.x {
            Size::Length(defx) => {
                out.x = defx + *x;
            }
            Size::Percent(defx) => {
                out.x = defx * rect.x + *x;
            }
        }
        match def.width {
            Size::Length(defw) => {
                out.width = defw;
            }
            Size::Percent(defw) => {
                out.width = defw * rect.width;
            }
        }
    }

    fn calc_vertical(
        metrics: &mut CalculationMetrics,
        rect: &Rect,
        out: &mut Rect,
        def: DefinitionRect,
    ) {
        match def.x {
            Size::Length(defx) => out.x = defx + metrics.offset_x,
            Size::Percent(defx) => out.x = defx * rect.width + metrics.offset_x,
        }
        match def.width {
            Size::Length(defw) => out.width = defw,
            Size::Percent(defw) => out.width = defw * rect.width,
        }

        match def.y {
            Size::Length(defy) => out.y = defy + metrics.offset_y,
            Size::Percent(defy) => out.y = defy * rect.height + metrics.offset_y,
        }
        match def.height {
            Size::Length(defh) => out.height = defh,
            Size::Percent(defh) => out.height = defh * rect.height,
        }

        metrics.largest_x = metrics.largest_x.max(out.width);
        metrics.offset_y += out.height;
        if metrics.offset_y >= rect.bottom() {
            metrics.offset_y = 0.0;
            metrics.offset_x += metrics.largest_x;
            metrics.largest_x = 0.0;
        }
    }

    fn calc_horizontal(
        x: &mut f32,
        y: &mut f32,
        rect: &Rect,
        out: &mut Rect,
        def: DefinitionRect,
        recip: f32,
    ) {
        match def.x {
            Size::Length(defx) => {
                out.x = defx + *x;
                *x += defx;
            }
            Size::Percent(defx) => {
                out.x = defx * rect.x * recip;
                *x += defx * rect.x;
            }
        }
        match def.width {
            Size::Length(defw) => {
                out.width = defw;
                *x += defw;
            }
            Size::Percent(defw) => {
                out.width = defw * rect.width;
                *x += defw * rect.width;
            }
        }

        match def.y {
            Size::Length(defy) => {
                out.y = (defy + *y) * recip;
            }
            Size::Percent(defy) => {
                out.x = defy * rect.y * recip;
            }
        }
        match def.height {
            Size::Length(defh) => {
                out.height = defh * recip;
            }
            Size::Percent(defh) => {
                out.height = defh * rect.height * recip;
            }
        }
    }

    fn calc_definition(
        metrics: &mut CalculationMetrics,
        corner: &Corner,
        direction: &Direction,
        def: DefinitionRect,
        rect: &Rect,
        offset_x: &mut f32,
        offset_y: &mut f32,
        recip: f32,
    ) -> Rect {
        let mut out = Rect::default();
        match direction {
            Direction::Vertical => Self::calc_vertical(metrics, rect, &mut out, def),
            Direction::Horizontal => {
                Self::calc_horizontal(offset_x, offset_y, rect, &mut out, def, recip)
            }
        }
        out
    }

    pub fn calculate(self, rect: Rect) -> Vec<Rect> {
        let mut x = 0.0;
        let mut y = 0.0;
        let recip = (self.boxes.len() as f32).recip();
        let mut out = Vec::with_capacity(self.boxes.len());
        let mut metrics = CalculationMetrics::default();
        for def in self.boxes {
            out.push(Self::calc_definition(
                &mut metrics,
                &self.corner,
                &self.direction,
                def,
                &rect,
                &mut x,
                &mut y,
                recip,
            ));
        }
        out
    }
}
