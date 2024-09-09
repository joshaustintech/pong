use raylib::prelude::*;

trait Shape {
    fn draw(&self, d: &mut RaylibDrawHandle<'_>);
}

trait Animation {
    fn update(&mut self, rl: &RaylibHandle);
}

#[derive(Clone)]
struct Coordinates {
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
    fn draw(&self, d: &mut RaylibDrawHandle<'_>) {
        d.draw_circle(self.location.x, self.location.y, self.radius, Color::WHITE);
    }
}

impl Animation for Ball {
    fn update(&mut self, rl: &RaylibHandle) {
        self.location.x += self.speed.x;
        self.location.y += self.speed.y;
        let radius: i32 = self.radius as i32;

        if self.location.y + radius >= rl.get_screen_height() || self.location.y - radius <= 0 {
            self.speed.y *= -1;
        }

        if self.location.x + radius >= rl.get_screen_width() || self.location.x - radius <= 0 {
            self.speed.x *= -1;
        }
    }
}

struct Paddle {
    location: Coordinates,
    size: Coordinates,
    speed: Coordinates
}

impl Shape for Paddle {
    fn draw(&self, d: &mut RaylibDrawHandle<'_>) {
        d.draw_rectangle(self.location.x, self.location.y, self.size.x, self.size.y, Color::WHITE);
    }
}

impl Animation for Paddle {
    fn update(&mut self, rl: &RaylibHandle) {
        if self.location.y + self.size.y >= rl.get_screen_height() {
            self.location.y = rl.get_screen_height() - self.size.y;
        }
        if self.location.y <= 0 {
            self.location.y = 0;
        }
    }
}

impl Paddle {
    fn update_cpu(&mut self, rl: &RaylibHandle, ball: Ball) {
        if ball.location.y + ball.radius as i32 >= self.location.y + self.size.y {
            self.location.y += self.speed.y;
        }
        if ball.location.y + ball.radius as i32 <= self.location.y {
            self.location.y -= self.speed.y;
        }
        self.update(&rl);
    }

    fn update_player(&mut self, rl: &RaylibHandle) {
        use raylib::consts::KeyboardKey::*;

        if rl.is_key_down(KEY_UP) {
            self.location.y = self.location.y - self.speed.y;
        }
        if rl.is_key_down(KEY_DOWN) {
            self.location.y = self.location.y + self.speed.y;
        }
        self.update(&rl);
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
    fn reset_ball(&mut self, rl: &RaylibHandle) {
        let speed_choices = [-1,1];
        self.ball.location.x = rl.get_screen_width() / 2;
        self.ball.location.y = rl.get_screen_height() / 2;
        let rand_x: i32 = rl.get_random_value(0..1);
        let rand_y: i32 = rl.get_random_value(0..1);
        self.ball.speed.x *= speed_choices[rand_x as usize];
        self.ball.speed.y *= speed_choices[rand_y as usize];
    }
}

impl Animation for Game {

    fn update(&mut self, rl: &RaylibHandle) {
        use raylib::ffi::*;

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
        if self.ball.location.x + self.ball.radius as i32 >= rl.get_screen_width() {
            self.player2_score += 1;
            self.reset_ball(&rl);
        }

        if self.ball.location.x - self.ball.radius as i32 <= 0 {
            self.player1_score += 1;
            self.reset_ball(&rl);
        }
    }
}

fn main() {
    let screen_width: i32 = 1280;
    let screen_height: i32 = 800;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Pong")
        .build();


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

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        // step 1: update
        game.update(&rl);
        game.ball.update(&rl);
        game.player1_paddle.update_player(&rl);
        game.player2_paddle.update_cpu(&rl, game.ball.clone());


        // step 2: draw
        let mut d = rl.begin_drawing(&thread);
        d.draw_line(screen_width / 2, 0, screen_width / 2, screen_height, Color::WHITE);
        game.ball.draw(&mut d);
        game.player1_paddle.draw(&mut d);
        game.player2_paddle.draw(&mut d);
        d.draw_text(game.player2_score.to_string().as_str(), screen_width / 4 - 20, 20, 80, Color::WHITE);
        d.draw_text(game.player1_score.to_string().as_str(), 3 * screen_width / 4 - 20, 20, 80, Color::WHITE);

        // step 3: clear
        d.clear_background(Color::BLACK);
    }

}
