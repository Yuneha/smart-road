use std::vec;
use sdl2::{ pixels::Color, rect::Point, render::Canvas, video::Window };
use crate::{ WINDOW_HEIGHT, WINDOW_WIDTH };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoadDirection {
    North,
    West,
    South,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Straight,
    Right,
}

#[derive(Debug, Clone)]
pub struct Road {
    pub lines1: Vec<Line>,
    pub lines2: Vec<Line>,
}

impl Road {
    pub fn new() -> Self {
        Road { lines1: vec![], lines2: vec![] }
    }

    pub fn render(canvas: &mut Canvas<Window>) {
        // Render road border
        canvas.set_draw_color(Color::RGB(250, 250, 250));
        canvas.draw_line(Point::new(362, 0), Point::new(362, 234)).unwrap();
        canvas.draw_line(Point::new(662, 0), Point::new(662, 234)).unwrap();

        canvas.draw_line(Point::new(662, 234), Point::new(WINDOW_WIDTH, 234)).unwrap();
        canvas.draw_line(Point::new(662, 534), Point::new(WINDOW_WIDTH, 534)).unwrap();

        canvas.draw_line(Point::new(662, 534), Point::new(662, WINDOW_HEIGHT)).unwrap();
        canvas.draw_line(Point::new(362, 534), Point::new(362, WINDOW_HEIGHT)).unwrap();

        canvas.draw_line(Point::new(362, 534), Point::new(0, 534)).unwrap();
        canvas.draw_line(Point::new(362, 234), Point::new(0, 234)).unwrap();
        // canvas.set_draw_color(Color::RGB(250, 250, 250));
        // canvas.draw_rect(Rect::new(362, 0, 300, WINDOW_HEIGHT as u32)).unwrap();
        // canvas.draw_rect(Rect::new(0, 234, WINDOW_WIDTH as u32, 300)).unwrap();

        // Render lines
        // Verticals
        // canvas.set_draw_color(Color::RGB(0, 0, 100));
        // canvas.draw_line(Point::new(412, 0), Point::new(412, WINDOW_HEIGHT)).unwrap();
        // canvas.draw_line(Point::new(462, 0), Point::new(462, WINDOW_HEIGHT)).unwrap();
        // canvas.draw_line(Point::new(512, 0), Point::new(512, WINDOW_HEIGHT)).unwrap();
        // canvas.draw_line(Point::new(562, 0), Point::new(562, WINDOW_HEIGHT)).unwrap();
        // canvas.draw_line(Point::new(612, 0), Point::new(612, WINDOW_HEIGHT)).unwrap();

        //Horizontals
        // canvas.set_draw_color(Color::RGB(0, 100, 0));
        // canvas.draw_line(Point::new(0, 284), Point::new(WINDOW_WIDTH, 284)).unwrap();
        // canvas.draw_line(Point::new(0, 334), Point::new(WINDOW_WIDTH, 334)).unwrap();
        // canvas.draw_line(Point::new(0, 384), Point::new(WINDOW_WIDTH, 384)).unwrap();
        // canvas.draw_line(Point::new(0, 434), Point::new(WINDOW_WIDTH, 434)).unwrap();
        // canvas.draw_line(Point::new(0, 484), Point::new(WINDOW_WIDTH, 484)).unwrap();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub road_direction: RoadDirection,
    pub direction: Direction,
}

impl Line {
    pub fn new(road_direction: RoadDirection, direction: Direction) -> Self {
        Line { road_direction, direction }
    }
}

#[derive(Debug)]
pub struct RoadIntersection {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl RoadIntersection {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        RoadIntersection { x, y, width, height }
    }
}
