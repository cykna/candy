use std::ops::Range;

use candy_shared_types::vec4f32_to_color;
use nalgebra::Vector4;
use skia_safe::{Paint, Point, Rect, SamplingOptions, canvas::SrcRectConstraint};

use crate::{
    BiDimensionalPainter,
    primitives::{CandyImage, CandySquare, CandyText},
};

pub use super::Candy2DefaultRenderer;
impl BiDimensionalPainter for Candy2DefaultRenderer {
    fn square(&mut self, square_info: &CandySquare) {
        let rule = &square_info.rule;

        let radius = rule.border_radius;
        let rect = {
            let position = square_info.position();
            let size = square_info.size();
            Rect::new(
                position.x,
                position.y,
                position.x + size.x,
                position.y + size.y,
            )
        };

        self.canvas()
            .draw_round_rect(rect, radius.x, radius.y, &rule.inner);
        let border_color = rule.border_color;

        if border_color.w == 0.0 || rule.border_width == 0.0 {
            return;
        }
        let mut paint = Paint::new(vec4f32_to_color(&border_color), None);
        paint
            .set_style(skia_safe::PaintStyle::Stroke)
            .set_stroke_width(rule.border_width);

        self.canvas()
            .draw_round_rect(&rect, radius.x, radius.y, &paint);
    }

    #[inline]
    fn circle(&mut self, position: &nalgebra::Vector2<f32>, color: &Vector4<f32>, radius: f32) {
        let paint = Paint::new(vec4f32_to_color(color), None);
        self.canvas()
            .draw_circle(Point::new(position.x, position.y), radius, &paint);
    }

    fn text_sliced(&mut self, info: &CandyText, range: Range<usize>) {
        let rule = &info.rule;

        let bounds = info.bounds();
        let canvas = self.canvas();
        canvas.save();
        canvas.clip_rect(
            Rect {
                left: bounds.x - info.font().size(),
                top: bounds.y - info.font().size(),
                right: bounds.x + bounds.width,
                bottom: bounds.y + bounds.height,
            },
            None,
            Some(true),
        );
        canvas.draw_str(
            &info.content()[range],
            Point::new(info.position().x, info.position().y),
            &info.font(),
            &rule.inner,
        );
        canvas.restore();
    }

    fn text(&mut self, info: &CandyText) {
        let rule = &info.rule;
        let canvas = self.canvas();
        canvas.save();
        let bounds = info.bounds();
        canvas.clip_rect(
            Rect {
                left: bounds.x - info.font().size(),
                top: bounds.y - info.font().size(),
                right: bounds.x + bounds.width,
                bottom: bounds.y + bounds.height,
            },
            None,
            Some(true),
        );
        canvas.draw_str(
            info.content(),
            Point::new(info.position().x, info.position().y),
            &info.font(),
            &rule.inner,
        );
        canvas.restore();
    }
    fn render_image(&mut self, image: &CandyImage) {
        let rule = &image.rule;
        let w = image.real_width();
        let h = image.real_height();
        let position = image.position();
        let rect = Rect::new(
            position.x,
            position.y,
            position.x + w as f32,
            position.y + h as f32,
        );

        let canvas = self.canvas();

        canvas.save();

        canvas.clip_rrect(
            &RRect::new_rect_xy(&rect, rule.border_radius.x, rule.border_radius.y),
            None,
            true,
        );

        canvas.draw_image_rect_with_sampling_options(
            image.image_handler(),
            Some((
                &Rect::new(0.0, 0.0, w as f32, h as f32),
                SrcRectConstraint::Fast,
            )),
            rect,
            SamplingOptions::default(),
            &rule.inner,
        );

        canvas.restore();
    }
    fn background(&mut self, color: &Vector4<f32>) {
        self.canvas().clear(*vec4f32_to_color(color));
    }
}
