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

    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
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

    fn calc_vertical(
        metrics: &mut CalculationMetrics,
        rect: &Rect,
        out: &mut Rect,
        def: DefinitionRect,
        gap: Vector2<f32>,
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
        metrics.offset_y += out.height + gap.y;
        if metrics.offset_y >= rect.bottom() {
            metrics.offset_y = 0.0;
            metrics.offset_x += metrics.largest_x + gap.x;
            metrics.largest_x = 0.0;
        }
    }

    fn calc_horizontal(
        metrics: &mut CalculationMetrics,
        rect: &Rect,
        out: &mut Rect,
        def: DefinitionRect,
        gap: Vector2<f32>,
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

        metrics.largest_y = metrics.largest_y.max(out.height);
        metrics.offset_x += out.width + gap.x;
        if metrics.offset_x >= rect.right() {
            metrics.offset_x = 0.0;
            metrics.offset_y += metrics.largest_y + gap.y;
            metrics.largest_y = 0.0;
        }
    }

    fn calc_definition(
        metrics: &mut CalculationMetrics,
        corner: &Corner,
        direction: &Direction,
        def: DefinitionRect,
        rect: &Rect,
        gap: Vector2<f32>,
    ) -> Rect {
        let mut out = Rect::default();
        match direction {
            Direction::Vertical => Self::calc_vertical(metrics, rect, &mut out, def, gap),
            Direction::Horizontal => Self::calc_horizontal(metrics, rect, &mut out, def, gap),
        }
        out
    }

    pub fn calculate_gap(&self, rect: &Rect) -> Vector2<f32> {
        let x = match self.gap.x {
            Size::Length(gx) => gx,
            Size::Percent(gx) => gx * rect.width,
        };
        let y = match self.gap.y {
            Size::Length(gy) => gy,
            Size::Percent(gy) => gy * rect.height,
        };
        Vector2::new(x, y)
    }

    pub fn calculate(self, rect: Rect) -> Vec<Rect> {
        let mut out = Vec::with_capacity(self.boxes.len());
        let mut metrics = CalculationMetrics::default();
        let gap = self.calculate_gap(&rect);
        for def in self.boxes {
            out.push(Self::calc_definition(
                &mut metrics,
                &self.corner,
                &self.direction,
                def,
                &rect,
                gap,
            ));
        }
        out
    }
}
