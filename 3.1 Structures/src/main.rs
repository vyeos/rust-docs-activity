// 1. Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).
// 2. Add a function square which takes a Point and a f32 as arguments, and returns a Rectangle with its top left corner on the point, and a width and height corresponding to the f32.

fn rect_area(
    Rectangle {
        top_left,
        bottom_right,
    }: Rectangle,
) -> f32 {
    let length = bottom_right.x - top_left.x;
    let width = top_left.y - bottom_right.y;
    length * width
}

fn square(Point { x, y }: Point, l: f32) -> Rectangle {
    Rectangle {
        top_left: Point { x, y },
        bottom_right: Point { x: x + l, y: y - l },
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug, Copy, Clone)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point {
        x: 10.3,
        ..another_point
    };

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    let my_rect_area = rect_area(rectangle);
    println!("Area of reactange {:?}: {:.2}", rectangle, my_rect_area);

    let square = square(point, 3.0);
    let my_square_area = rect_area(square);
    println!("Area of Square {:?}: {:.2}", square, my_square_area);
}
