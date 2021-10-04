use embedded_graphics::{
    geometry::AnchorPoint,
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{
        circle::Circle, line::Line, rectangle::Rectangle, triangle::Triangle, PrimitiveStyle,
        PrimitiveStyleBuilder, Styled,
    },
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
            Self::Circ(circ) => circ.primitive.diameter / 2,
            Self::Tri(tri) => {
                ((tri.primitive.vertices[2].x - tri.primitive.vertices[1].x) / 2) as u32
            }
            Self::Rect(rect) => rect.primitive.size.height / 2,
            Self::X(x) => ((x[0].primitive.end.y - x[0].primitive.start.y).abs() / 2) as u32,
        }
    }

    pub fn draw(&self, disp: &mut Framebuffer) {
        match self {
            Self::Circ(circ) => circ.draw(disp).unwrap(),
            Self::Tri(tri) => tri.draw(disp).unwrap(),
            Self::Rect(rect) => rect.draw(disp).unwrap(),
            Self::X(x) => {
                for line in x {
                    line.draw(disp).unwrap();
                }
            }
        };
    }

    pub fn translate_mut(&mut self, point: Point) {
        match self {
            Self::Circ(circ) => {
                circ.translate_mut(point);
            }
            Self::Tri(tri) => {
                tri.translate_mut(point);
            }
            Self::Rect(rect) => {
                rect.translate_mut(point);
            }
            Self::X(x) => {
                for line in x {
                    line.translate_mut(point);
                }
            }
        };
    }

    pub fn grow(&mut self) {
        let size = self.size();
        if size < 31 {
            match self {
                Self::Circ(circ) => {
                    circ.primitive.diameter += 2;
                    circ.primitive = circ.primitive.translate(Point::new(-1, -1));
                }
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
                Self::Circ(circ) => {
                    circ.primitive.diameter -= 2;
                    circ.primitive = circ.primitive.translate(Point::new(1, 1));
                }
                Self::Tri(tri) => shrink_triangle(tri),
                Self::Rect(rect) => shrink_rectangle(rect),
                Self::X(x) => shrink_x(x),
            }
        }
    }

    pub fn center(&self) -> Point {
        match self {
            Self::Circ(circ) => circ.primitive.center(),
            Self::Tri(tri) => Point::new(
                tri.primitive.vertices[0].x,
                (tri.primitive.vertices[0].y + tri.primitive.vertices[1].y) / 2,
            ),
            Self::Rect(rect) => rect.primitive.center(),

            Self::X(x) => Point::new(
                (x[0].primitive.start.x + x[0].primitive.end.x) / 2,
                (x[0].primitive.start.y + x[0].primitive.end.y) / 2,
            ),
        }
    }

    pub fn move_by(&mut self, delta_x_pixels: i32, delta_y_pixels: i32, max_x: i32, max_y: i32) {
        let existing_center = self.center();
        let requested_delta = Point::new(delta_x_pixels, delta_y_pixels);
        let mut target_location: Point = existing_center + requested_delta;
        target_location.x = target_location.x.clamp(0, max_x);
        target_location.y = target_location.y.clamp(0, max_y);
        let actual_delta = target_location - existing_center;
        self.translate_mut(actual_delta);
    }

    pub fn new_circle(point: Point, size: u32) -> Self {
        Self::Circ(
            Circle::with_center(point, size * 2).into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_color(Rgb888::RED)
                    .stroke_width(1)
                    .build(),
            ),
        )
    }

    pub fn new_triangle(point: Point, size: u32) -> Self {
        let mut tri = Triangle::new(point, point, point).into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(Rgb888::GREEN)
                .stroke_width(1)
                .build(),
        );
        grow_triangle_by(&mut tri, size as _);
        Self::Tri(tri)
    }

    pub fn new_rectangle(point: Point, size: u32) -> Self {
        let mut rect = Rectangle::new(point, Size::zero()).into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(Rgb888::MAGENTA)
                .stroke_width(1)
                .build(),
        );
        grow_rectangle_by(&mut rect, (size * 2) as _);
        Self::Rect(rect)
    }

    pub fn new_x(point: Point, size: u32) -> Self {
        let line_1 = Line::new(point, point).into_styled(
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
    grow_triangle_by(styled_tri, 2)
}

fn shrink_triangle(styled_tri: &mut StyledTri) {
    grow_triangle_by(styled_tri, -2)
}

fn grow_triangle_by(styled_tri: &mut StyledTri, grow_by: i32) {
    let mut tri = &mut styled_tri.primitive;
    tri.vertices[0].y -= grow_by;

    tri.vertices[1].x -= grow_by;
    tri.vertices[1].y += grow_by;

    tri.vertices[2].x += grow_by;
    tri.vertices[2].y += grow_by;
}

fn grow_rectangle(styled_rect: &mut StyledRect) {
    grow_rectangle_by(styled_rect, 2)
}

fn shrink_rectangle(styled_rect: &mut StyledRect) {
    grow_rectangle_by(styled_rect, -2)
}

fn grow_rectangle_by(styled_rect: &mut StyledRect, grow_by: i32) {
    let rect = styled_rect.primitive;
    let new_rect = rect.resized(
        Size::new(
            (rect.size.width as i32 + grow_by) as u32,
            (rect.size.height as i32 + grow_by) as u32,
        ),
        AnchorPoint::Center,
    );

    styled_rect.primitive = new_rect;
}

fn grow_x(styled_x: &mut StyledX) {
    grow_x_by(styled_x, 2)
}

fn shrink_x(styled_x: &mut StyledX) {
    grow_x_by(styled_x, -2)
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
