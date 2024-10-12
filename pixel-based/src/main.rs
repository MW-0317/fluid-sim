use std::{env::var, iter};

use nannou::{draw::{background::new, properties::stroke}, prelude::*, rand::{thread_rng, Rng}};

#[derive(Debug)]
struct Area {
    n: usize,
    rows: usize,
    columns: usize,
    w: f32,
    h: f32,
    grid: Vec<Vec<f32>>,
    velocity: Vec<Vec<Vec2>>,
}

impl Area {
    fn new(rect: Rect, n: usize) -> Self {
        let rows = n;
        let columns = n;
        let w = rect.w() / n as f32;
        let h = rect.h() / n as f32;
        let grid = vec![vec![0.0; rows]; columns];
        let velocity = vec![vec![Vec2::new(0.0, 0.0); rows]; columns];

        let mut area = Area {
            n,
            rows,
            columns,
            w,
            h,
            grid,
            velocity
        };

        area.init();

        area
    }

    fn init(&mut self) {
        let mut t_rng = thread_rng();
        self.grid[self.columns/2][self.rows/2] = 100.0;
        self.velocity[self.columns/2][self.rows/2] = Vec2::new(-5.0, 1.0);
        // for i in 0..self.columns {
        //     for j in 0..self.rows {
        //         self.grid[i][j] = t_rng.gen_range(0.0..1.0);
        //     }
        // }
    }

    fn generate(&mut self) {
        let mut t_rng = thread_rng();
        self.iterate(20, Some(2.0));
    }

    fn iterate(&mut self, iterations: i32, k: Option<f32>) {
        let k = k.unwrap_or(1.0);
        let mut temp_grid = self.grid.clone();
        for i in 0..iterations {
            for x in 0..self.columns {
                for y in 0..self.rows {
                    temp_grid[x][y] = (1.0/(1.0+k)) * (self.grid[x][y] + k as f32 * (
                        (
                            temp_grid[if x+1 >= self.columns    {x-1} else {x+1}][y] + 
                            temp_grid[if x as i32-1 <= 0               {x+1} else {x-1}][y] + 
                            temp_grid[x][if y+1 >= self.rows    {y-1} else {y+1}] + 
                            temp_grid[x][if y as i32-1 <= 0            {y+1} else {y-1}]
                        )
                        /
                        4.0
                    ));
                }
            }
        }
        self.grid = temp_grid;
    }

    fn display(&self, draw: &Draw, rect: &Rect) {
        for i in 0..self.columns {
            for j in 0..self.rows {
                let mut fill = self.grid[i][j];
                let offset = self.w as f32 / 2.0;
                draw.rect()
                    .x_y(
                        offset + (i as f32 * self.w) - rect.right() as f32, 
                        offset + (j as f32 * self.h) - rect.top() as f32)
                    .w_h(self.w as f32, self.h as f32)
                    .gray(fill)
                    .stroke(BLACK);

                let arr = Vec2::new(
                    offset + (i as f32 * self.w) - rect.right() as f32, 
                    offset + (j as f32 * self.h) - rect.top() as f32
                );
                draw.arrow()
                    .weight(self.w * 0.3)
                    .head_width(self.w * 0.5)
                    .color(RED)
                    .start(arr)
                    .end(arr + self.velocity[i][j] * self.w);
            }
        }
    }
}

struct Model {
    area: Area
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn model(_app: &App) -> Model {
    let rect = Rect::from_w_h(400.0, 400.0);
    let area = Area::new(rect, 51);

    _app.new_window()
        .size(rect.w() as u32, rect.h() as u32)
        .view(view)
        .build()
        .unwrap();

    Model {
        area
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.area.generate();
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    let draw = _app.draw();
    draw.background().color(WHITE);

    _model.area.display(&draw, &_app.window_rect());
    draw.to_frame(_app, &_frame).unwrap();
}
