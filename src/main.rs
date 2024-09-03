use raylib_ffi::{colors::{BLACK, WHITE}, enums, BeginDrawing, CheckCollisionCircleRec, ClearBackground, CloseWindow, DrawCircle, DrawLine, DrawText, DrawRectangle, EndDrawing, GetRandomValue, GetScreenHeight, GetScreenWidth, InitWindow, IsKeyDown, Rectangle, SetTargetFPS, TextFormat, Vector2, WindowShouldClose};
use std::{ffi::CString, ops::Index};

trait Shape {
    fn draw(&self);
}

trait Animation {
    fn update(&mut self);
}

#[derive(Clone)]
struct Coordinates { // TODO: replace and refactor with builtin Vector2
    x: i32,
    y: i32
}

#[derive(Clone)]
struct Ball {
    location: Coordinates,
    radius: f32,
    speed: Coordinates
}

impl Shape for Ball {
    fn draw(&self) {
        unsafe {
            DrawCircle(self.location.x, self.location.y, self.radius, WHITE);
        }
    }
}

impl Animation for Ball {
    fn update(&mut self) {
        self.location.x += self.speed.x;
        self.location.y += self.speed.y;
        let radius: i32 = self.radius as i32;

        unsafe {
            if self.location.y + radius >= GetScreenHeight() || self.location.y - radius <= 0 {
                self.speed.y *= -1;
            }

            if self.location.x + radius >= GetScreenWidth() || self.location.x - radius <= 0 {
                self.speed.x *= -1;
            }
        }
    }
}

struct Paddle { // TODO: refactor using Rectangle
    location: Coordinates,
    size: Coordinates,
    speed: Coordinates
}

impl Shape for Paddle {
    fn draw(&self) {
        unsafe {
            DrawRectangle(self.location.x, self.location.y, self.size.x, self.size.y, WHITE);
        }
    }
}

impl Animation for Paddle {
    fn update(&mut self) {
        unsafe {
            if self.location.y + self.size.y >= GetScreenHeight() {
                self.location.y = GetScreenHeight() - self.size.y;
            }
        }
        if self.location.y <= 0 {
            self.location.y = 0;
        }
    }
}

impl Paddle {
    fn update_cpu(&mut self, ball: Ball) {
        if ball.location.y + ball.radius as i32 >= self.location.y + self.size.y {
            self.location.y += self.speed.y;
        }
        if ball.location.y + ball.radius as i32 <= self.location.y {
            self.location.y -= self.speed.y;
        }
        self.update();
    }

    fn update_player(&mut self) {
        unsafe {
            if IsKeyDown(enums::KeyboardKey::Up as i32) {
                self.location.y = self.location.y - self.speed.y;
            }
            if IsKeyDown(enums::KeyboardKey::Down as i32) {
                self.location.y = self.location.y + self.speed.y;
            }
        }
        self.update();
    }
}

struct Game {
    player1_paddle: Paddle,
    player1_score: i32,
    player2_paddle: Paddle,
    player2_score: i32,
    ball: Ball
}

impl Game {
    fn reset_ball(&mut self) {
        let speed_choices = [-1,1];
        unsafe {
            self.ball.location.x = GetScreenWidth() / 2;
            self.ball.location.y = GetScreenHeight() / 2;
            let rand_x = GetRandomValue(0, 1) as usize;
            let rand_y = GetRandomValue(0, 1) as usize;
            self.ball.speed.x *= speed_choices.index(rand_x);
            self.ball.speed.y *= speed_choices.index(rand_y);
        }
    }
}

impl Animation for Game {

    fn update(&mut self) {
        // check for collisions
        let center = Vector2 {
            x: self.ball.location.x as f32,
            y: self.ball.location.y as f32
        };

        let player_rect = Rectangle {
            x: self.player1_paddle.location.x as f32,
            y: self.player1_paddle.location.y as f32,
            width: self.player1_paddle.size.x as f32,
            height: self.player1_paddle.size.y as f32
        };

        let cpu_rect = Rectangle {
            x: self.player2_paddle.location.x as f32,
            y: self.player2_paddle.location.y as f32,
            width: self.player2_paddle.size.x as f32,
            height: self.player2_paddle.size.y as f32
        };

        unsafe {
            if CheckCollisionCircleRec(center, self.ball.radius, player_rect) ||
                CheckCollisionCircleRec(center, self.ball.radius, cpu_rect) {
                self.ball.speed.x *= -1;
            }
        }

        // check for scoring
        unsafe {
            if self.ball.location.x + self.ball.radius as i32 >= GetScreenWidth() {
                self.player2_score += 1;
                self.reset_ball();
            }
            if self.ball.location.x - self.ball.radius as i32 <= 0 {
                self.player1_score += 1;
                self.reset_ball();
            }
        }
    }
}

fn main() {
    let screen_width = 1280;
    let screen_height = 800;
    let window_title = CString::new("Pong").unwrap();


    let paddle_size = Coordinates { x: 25, y: 120 };

    let mut game = Game {
        player1_paddle: Paddle {
            location: Coordinates { x: screen_width - 35, y: screen_height / 2 - 60 },
            size: paddle_size.clone(),
            speed: Coordinates { x: 0, y: 7 }
        },
        player1_score: 0,
        player2_paddle: Paddle {
            location: Coordinates { x: 10, y: screen_height / 2 - 60 },
            size: paddle_size.clone(),
            speed: Coordinates { x: 0, y: 5 }
        },
        player2_score: 0,
        ball: Ball {
            location: Coordinates { x: screen_width / 2, y: screen_height / 2 },
            radius: 20.0,
            speed: Coordinates { x: 7, y: 7 }
        }
    };

    unsafe {
        InitWindow(screen_width, screen_height, window_title.as_ptr());
        SetTargetFPS(60);

        while !WindowShouldClose() {

            // updates
            game.update();
            game.ball.update();
            game.player1_paddle.update_player();
            game.player2_paddle.update_cpu(game.ball.clone());

            // drawing
            ClearBackground(BLACK);
            BeginDrawing();
            DrawLine(screen_width / 2, 0, screen_width / 2, screen_height, WHITE);
            game.ball.draw();
            game.player1_paddle.draw();
            game.player2_paddle.draw();
            DrawText(TextFormat(CString::new("%i").unwrap().as_ptr(), game.player2_score), screen_width / 4 - 20, 20, 80, WHITE);
            DrawText(TextFormat(CString::new("%i").unwrap().as_ptr(), game.player1_score), 3 * screen_width / 4 - 20, 20, 80, WHITE);
            EndDrawing()

        }

        CloseWindow();

    }
}
