use nalgebra::{Vector2, Vector4};

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
        Size::Length(0.0)
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

impl CalculationMetrics {
    pub fn new(offset_x: f32, offset_y: f32) -> Self {
        Self {
            offset_x,
            offset_y,
            largest_x: 0.0,
            largest_y: 0.0,
        }
    }
}

#[derive(Default)]
pub struct Layout {
    pub(crate) boxes: Vec<DefinitionRect>,
    gap: Vector2<Size>,
    corner: Corner,
    direction: Direction,
    padding: Vector4<Size>,
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

    pub fn with_direction(&mut self, direction: Direction) -> &mut Self {
        self.direction = direction;
        self
    }

    pub fn with_corner(&mut self, corner: Corner) -> &mut Self {
        self.corner = corner;
        self
    }

    pub fn with_definition(&mut self, def: DefinitionRect) -> &mut Self {
        self.boxes.push(def);
        self
    }

    pub fn with_gap(&mut self, gap: Vector2<Size>) -> &mut Self {
        self.gap = gap;
        self
    }

    pub fn with_padding(&mut self, padding: Vector4<Size>) -> &mut Self {
        self.padding = padding;
        self
    }

    ///Calculates a Rect on the vertical based on the parent `rect`
    fn calc_vertical(
        metrics: &mut CalculationMetrics,
        rect: &Rect,
        out: &mut Rect,
        def: &DefinitionRect,
        gap: Vector2<f32>,
    ) {
        out.x = match def.x {
            Size::Length(defx) => defx + metrics.offset_x,
            Size::Percent(defx) => defx * rect.width + metrics.offset_x,
        };
        out.width = match def.width {
            Size::Length(defw) => defw,
            Size::Percent(defw) => defw * rect.width,
        };

        out.y = match def.y {
            Size::Length(defy) => defy + metrics.offset_y,
            Size::Percent(defy) => defy * rect.height + metrics.offset_y,
        };
        out.height = match def.height {
            Size::Length(defh) => defh,
            Size::Percent(defh) => defh * rect.height,
        };

        metrics.largest_x = metrics.largest_x.max(out.width);

        metrics.offset_y += out.height + gap.y;
        if metrics.offset_y > rect.bottom() {
            out.y = rect.y;
            out.x += metrics.largest_x + gap.x;
            metrics.offset_y = rect.y + out.height + gap.y;
            metrics.offset_x += metrics.largest_x + gap.x;
            metrics.largest_x = 0.0;
        }
    }

    ///Calculates a Rect on the horizontal based on the parent `rect`
    fn calc_horizontal(
        metrics: &mut CalculationMetrics,
        rect: &Rect,
        out: &mut Rect,
        def: &DefinitionRect,
        gap: Vector2<f32>,
    ) {
        out.x = match def.x {
            Size::Length(defx) => defx + metrics.offset_x,
            Size::Percent(defx) => defx * rect.width + metrics.offset_x,
        };
        out.width = match def.width {
            Size::Length(defw) => defw,
            Size::Percent(defw) => defw * rect.width,
        };

        out.y = match def.y {
            Size::Length(defy) => defy + metrics.offset_y,
            Size::Percent(defy) => defy * rect.height + metrics.offset_y,
        };
        out.height = match def.height {
            Size::Length(defh) => defh,
            Size::Percent(defh) => defh * rect.height,
        };

        metrics.largest_y = metrics.largest_y.max(out.height);

        metrics.offset_x += out.width + gap.x;

        if metrics.offset_x > rect.right() {
            out.x = rect.x;
            out.y += metrics.largest_y + gap.y;
            metrics.offset_x = rect.x + out.width + gap.x;
            metrics.offset_y += metrics.largest_y + gap.y;
            metrics.largest_y = 0.0;
        }
    }

    ///Based on the direction of the layout and the `def` calculates a rect that corresponds, to the order it's being created
    ///and the size of the parent `rect`
    fn calc_definition(
        metrics: &mut CalculationMetrics,
        corner: &Corner,
        direction: &Direction,
        def: &DefinitionRect,
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

    ///Calculates the padding in pixels of this Layout
    pub fn calculate_padding(&self, rect: &Rect) -> Vector4<f32> {
        let x = match self.padding.x {
            Size::Length(x) => x,
            Size::Percent(x) => rect.width * x,
        };
        let y = match self.padding.y {
            Size::Length(y) => y,
            Size::Percent(y) => rect.height * y,
        };

        let r = match self.padding.z {
            Size::Length(z) => z,
            Size::Percent(z) => rect.width * z,
        };
        let b = match self.padding.w {
            Size::Length(w) => w,
            Size::Percent(w) => rect.height * w,
        };
        Vector4::new(x, y, r, b)
    }

    ///Calculates the gap in Pixels of this Layout
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

    ///Calculates this layout based on its values and the boxes defined. Note that it will generate N Rects, where N is the amount of boxes added before calculating it.
    ///The boxes are in order of pushing, so the Nth Rect on the out vector is correspondent to the Nth push
    pub fn calculate(&self, mut rect: Rect) -> Vec<Rect> {
        let mut out = Vec::with_capacity(self.boxes.len());

        {
            let padding = self.calculate_padding(&rect);
            rect.x += padding.x;
            rect.y += padding.y;
            rect.width -= padding.z + padding.x;
            rect.height -= padding.w + padding.y;
        }

        let gap = self.calculate_gap(&rect);
        let mut metrics = CalculationMetrics::new(rect.x, rect.y);

        for def in &self.boxes {
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
