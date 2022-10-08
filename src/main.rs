use std::ops::Not;
use nannou::prelude::*;

const WIDTH: u32 = 12;
const MAX_X: i32 = (WIDTH as i32 / 2) - 1;
const MIN_X: i32 = 1 - (WIDTH as i32 / 2);
const HEIGHT: u32 = 12;
const MAX_Y: i32 = (HEIGHT as i32 / 2) - 1;
const MIN_Y: i32 = 1 - (HEIGHT as i32 / 2);

const SIZE: u32 = 64;
const DELAY: f64 = 0.16;

const HEAD_COLOR: (f32, f32, f32) = (0.85, 1.00, 0.80);
const TAIL_COLOR: (f32, f32, f32) = (0.08, 0.50, 0.25);
const FOOD_COLOR: (f32, f32, f32) = (0.65, 0.70, 0.02);

const BG_COLOR_1: (f32, f32, f32) = (0.00, 0.07, 0.10);
const BG_COLOR_2: (f32, f32, f32) = (0.00, 0.10, 0.10);

fn c(color: (f32, f32, f32)) -> Rgb<f32> {
    rgb(color.0, color.1, color.2)
}

fn main() {
    nannou::app(new_model).update(update).run();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Into<Vec2> for Point {
    fn into(self) -> Vec2 {
        pt2(self.x as f32 * SIZE as f32, self.y as f32 * SIZE as f32)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Still,
    Left,
    Right,
    Up,
    Down,
}

impl Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Direction::Still => self,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
}

#[derive(Clone, Debug)]
struct Model {
    pub head: Point,
    pub tail: Vec<Point>,
    pub food: Point,
    pub direction: Direction,
    pub last_move: f64,
    pub queue: Vec<Direction>,
}

fn new_model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH * SIZE, HEIGHT * SIZE)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model {
        head: Point { x: 0, y: 0 },
        tail: Vec::new(),
        food: Point { x: MAX_X / 2, y: 0 },
        direction: Direction::Still,
        last_move: 0.0,
        queue: Vec::new(),
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Left | Key::A => model.queue.insert(0, Direction::Left),
        Key::Right | Key::D => model.queue.insert(0, Direction::Right),
        Key::Up | Key::W => model.queue.insert(0, Direction::Up),
        Key::Down | Key::S => model.queue.insert(0, Direction::Down),
        _ => {}
    }

    if model.queue.len() != 0 && model.queue[0] == !model.direction && model.tail.len() != 0 {
        model.queue.pop();
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    model.last_move += update.since_last.as_secs_f64();
    if model.last_move < DELAY {
        return;
    }
    model.last_move = 0.0;

    model.direction = model.queue.pop().unwrap_or(model.direction);

    if model.tail.pop().is_some() {
        model.tail.insert(0, model.head);
    }

    match model.direction {
        Direction::Left => model.head.x -= 1,
        Direction::Right => model.head.x += 1,
        Direction::Up => model.head.y += 1,
        Direction::Down => model.head.y -= 1,
        Direction::Still => {}
    }

    if model.tail.contains(&model.head) {
        app.quit();
    }

    // TODO: is it worth reworking the tail to allow scrolling?
    if model.head.x > MAX_X {
        // model.head.x = -9;
        app.quit();
    } else if model.head.x < MIN_X {
        // model.head.x = 9;
        app.quit();
    } else if model.head.y > MAX_Y {
        // model.head.y = -9;
        app.quit();
    } else if model.head.y < MIN_Y {
        // model.head.y = 9;
        app.quit();
    }

    if model.head == model.food {
        let mut food_candidate = Point {
            x: random_range(MIN_X, MAX_X),
            y: random_range(MIN_Y, MAX_Y),
        };
        while model.tail.contains(&food_candidate) || food_candidate == model.head {
            food_candidate = Point {
                x: random_range(MIN_X, MAX_X),
                y: random_range(MIN_Y, MAX_Y),
            };
        }

        model.food = food_candidate;

        model.tail.insert(0, model.head);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let mut draw = app.draw();
    background(&mut draw, c(BG_COLOR_1), c(BG_COLOR_2));

    draw.ellipse()
        .width(SIZE as f32 - 2.5)
        .height(SIZE as f32 - 2.5)
        .color(c(FOOD_COLOR))
        .xy(model.food.into());

    for (i, &segment) in model.tail.iter().enumerate() {
        if i == 0 {
            draw.line()
                .weight(SIZE as f32 - 5.0)
                .color(c(TAIL_COLOR))
                .caps_round()
                .start(model.head.into())
                .end(segment.into());
        } else if i == model.tail.len() - 1 {
            draw.line()
                .weight(SIZE as f32 - 10.0)
                .color(c(TAIL_COLOR))
                .caps_round()
                .start(model.tail[i - 1].into())
                .end(segment.into());
        } else if i == model.tail.len() - 2 {
            draw.line()
                .weight(SIZE as f32 - 5.0)
                .color(c(TAIL_COLOR))
                .caps_round()
                .start(model.tail[i - 1].into())
                .end(segment.into());
        } else {
            draw.line()
                .weight(SIZE as f32 - 5.0)
                .color(c(TAIL_COLOR))
                .caps_round()
                .start(model.tail[i - 1].into())
                .end(segment.into());

            draw.line()
                .weight(SIZE as f32 - 5.0)
                .color(c(TAIL_COLOR))
                .caps_round()
                .start(model.tail[i + 1].into())
                .end(segment.into());
        }
    }

    draw.ellipse()
        .width(SIZE as f32 + 2.0)
        .height(SIZE as f32 + 2.0)
        .color(c(HEAD_COLOR))
        .xy(model.head.into());

    draw.to_frame(app, &frame).unwrap();
}

fn background(draw: &mut Draw, c1: Rgb<f32>, c2: Rgb<f32>) {
    for y in (1 - (HEIGHT as i32 / 2))..(HEIGHT as i32 / 2) {
        for x in (1 - (WIDTH as i32 / 2))..(WIDTH as i32 / 2) {
            draw.rect()
                .width(SIZE as f32)
                .height(SIZE as f32)
                .x_y((x * SIZE as i32) as f32, (y * SIZE as i32) as f32)
                .color(if (x + y) % 2 == 0 { c1 } else { c2 });
        }
    }
}
