use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{circle::Circle, rectangle::Rectangle, triangle::Triangle, line::Line},
    style::{PrimitiveStyle, PrimitiveStyleBuilder, Styled},
};

use psp::embedded_graphics::Framebuffer;

type StyledRect = Styled<Rectangle, PrimitiveStyle<Rgb888>>;
type StyledCirc = Styled<Circle, PrimitiveStyle<Rgb888>>;
type StyledTri = Styled<Triangle, PrimitiveStyle<Rgb888>>;
type StyledX = [Styled<Line, PrimitiveStyle<Rgb888>>; 2];

pub enum DrawObject {
    Circ(StyledCirc),
    Rect(StyledRect),
    Tri(StyledTri),
    X(StyledX),
}

impl DrawObject {
    pub fn size(&self) -> u32 {
        match self {
            Self::Circ(circ) => circ.primitive.radius,
            Self::Tri(tri) => tri.primitive.size().height / 2,
            Self::Rect(rect) => rect.primitive.size().height / 2,
            Self::X(x) => x[0].primitive.size().height / 2,
        }
    }

    pub fn draw(&self, disp: &mut Framebuffer) {
        match self {
            Self::Circ(circ) => circ.draw(disp).unwrap(),
            Self::Tri(tri) => tri.draw(disp).unwrap(),
            Self::Rect(rect) => rect.draw(disp).unwrap(),
            Self::X(x) => { for line in x { line.draw(disp).unwrap(); } },
        };
    }

    pub fn translate_mut(&mut self, point: Point) {
        match self {
            Self::Circ(circ) => {circ.translate_mut(point);},
            Self::Tri(tri) => {tri.translate_mut(point);},
            Self::Rect(rect) => {rect.translate_mut(point);},
            Self::X(x) => { for line in x { line.translate_mut(point); } },
        };
    }

    pub fn grow(&mut self) {
        let size = self.size();
        if size < 31 {
            match self {
                Self::Circ(circ) => circ.primitive.radius += 1,
                Self::Tri(tri) => grow_triangle(tri),
                Self::Rect(rect) => grow_rectangle(rect),
                Self::X(x) => grow_x(x),
            }
        }
    }

    pub fn shrink(&mut self) {
        let size = self.size();
        if size > 2 {
            match self {
                Self::Circ(circ) => circ.primitive.radius -= 1,
                Self::Tri(tri) => shrink_triangle(tri),
                Self::Rect(rect) => shrink_rectangle(rect),
                Self::X(x) => shrink_x(x),
            }
        }
    }

    pub fn center(&self) -> Point {
        match self {
            Self::Circ(circ) => circ.primitive.center,
            Self::Tri(tri) => Point::new(
                tri.primitive.p1.x,
                (tri.primitive.p1.y + tri.primitive.p2.y) / 2,
            ),
            Self::Rect(rect) => Point::new(
                (rect.primitive.top_left.x + rect.primitive.bottom_right.x) / 2,
                (rect.primitive.top_left.y + rect.primitive.bottom_right.y) / 2,
            ),

            Self::X(x) => Point::new(
                (x[0].primitive.start.x + x[0].primitive.end.x) / 2,
                (x[0].primitive.start.y + x[0].primitive.end.y) / 2,
            ),
        }
    }

    pub fn new_circle(point: Point, size: u32) -> Self {
        Self::Circ(Circle::new(point, size).into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(Rgb888::RED)
                .stroke_width(1)
                .build(),
        ))
    }

    pub fn new_triangle(point: Point, size: u32) -> Self {
        let mut tri = Triangle::new(
            point,
            point,
            point,
        )
        .into_styled(
            PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::GREEN)
            .stroke_width(1)
            .build(),
        );
        grow_triangle_by(&mut tri, size as _);
        Self::Tri(tri)
    }

    pub fn new_rectangle(point: Point, size: u32) -> Self {
        let mut rect = Rectangle::new(
            point,
            point,
        )
        .into_styled(
            PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::MAGENTA)
            .stroke_width(1)
            .build(),
        );
        grow_rectangle_by(&mut rect, size as _);
        Self::Rect(rect)
    }

    pub fn new_x(point: Point, size: u32) -> Self {
        let line_1 = Line::new(
            point,
            point,
        ).into_styled(
            PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::BLUE)
            .stroke_width(1)
            .build(),
        );
        let line_2 = line_1.clone();
        let mut x = [line_1, line_2];
        grow_x_by(&mut x, size as _);
        Self::X(x)
    }
}

fn grow_triangle(styled_tri: &mut StyledTri) {
    grow_triangle_by(styled_tri, 1)
}

fn shrink_triangle(styled_tri: &mut StyledTri) {
    grow_triangle_by(styled_tri, -1)
}

fn grow_triangle_by(styled_tri: &mut StyledTri, grow_by: i32) {
    let mut tri = &mut styled_tri.primitive;
    tri.p1.y -= grow_by;

    tri.p2.x -= grow_by;
    tri.p2.y += grow_by;

    tri.p3.x += grow_by;
    tri.p3.y += grow_by;
}

fn grow_rectangle(styled_rect: &mut StyledRect) {
    grow_rectangle_by(styled_rect, 1)
}

fn shrink_rectangle(styled_rect: &mut StyledRect) {
    grow_rectangle_by(styled_rect, -1)
}

fn grow_rectangle_by(styled_rect: &mut StyledRect, grow_by: i32) {
    let mut rect = &mut styled_rect.primitive;
    rect.top_left.x -= grow_by;
    rect.top_left.y -= grow_by;

    rect.bottom_right.x += grow_by;
    rect.bottom_right.y += grow_by;
}

fn grow_x(styled_x: &mut StyledX) {
    grow_x_by(styled_x, 1)
}

fn shrink_x(styled_x: &mut StyledX) {
    grow_x_by(styled_x, -1)
}

fn grow_x_by(styled_x: &mut StyledX, grow_by: i32) {
    let mut line_1 = &mut styled_x[0].primitive;
    line_1.start.x -= grow_by;
    line_1.start.y -= grow_by;
    line_1.end.x += grow_by;
    line_1.end.y += grow_by;

    let mut line_2 = &mut styled_x[1].primitive;
    line_2.start.x -= grow_by;
    line_2.start.y += grow_by;
    line_2.end.x += grow_by;
    line_2.end.y -= grow_by;
}
