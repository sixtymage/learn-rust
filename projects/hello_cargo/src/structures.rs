#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

// tuple
struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
  x: f32,
  y: f32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
  top_left: Point,
  bottom_right: Point,
}

fn rect_area(r: Rectangle) -> f32 {
  let Rectangle {
    top_left: Point { x: left_edge, y: top_edge },
    bottom_right: Point {x: right_edge, y: bottom_edge}
  } = r;

  return (right_edge - left_edge) * (bottom_edge-top_edge);
}

fn square(top_left: Point, length: f32) -> Rectangle {
  let Point {x: left, y: top }= top_left;
  return Rectangle {
    top_left: top_left, 
    bottom_right: Point {
      x: left+length, 
      y: top+length,
    }
  };
}

pub fn exercise_structs() {
  let name = String::from("Peter");
  let age = 32;
  let peter = Person { name, age };

  println!("{:?}", peter);

  let point: Point = Point { x: 3.2, y: 4.1 };
  println!("Point coords: ({}, {})", point.x, point.y);

  let bottom_right: Point = Point { x: 5.2, ..point };
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

  println!("Rect: {:?}", rectangle);

  // unit
  let _unit: Unit = Unit;

  let pair: Pair = Pair(3, 4.3);
  println!("Pair contains {:?} and {:?} ", pair.0, pair.1);

  let Pair(integer, decimal) = pair;
  println!("Pair contains {:?} and {:?} ", integer, decimal);

  // area
  let r: Rectangle = Rectangle { top_left: Point{x: 2.0, y: 10.0}, bottom_right: Point{x: 4.0, y: 13.0}};
  let area = rect_area(r);
  println!("rect_area: {}", area);

  let s: Rectangle = square(Point{x: 1.0, y: 1.0}, 4.0);
  println!("Square: {:?}", s);

}
