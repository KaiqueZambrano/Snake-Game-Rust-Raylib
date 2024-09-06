use std::process::exit;
use raylib::prelude::*;
use rand::Rng;

struct Position {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

struct Food {
    x: i32,
    y: i32,
}

trait SnakeBehaviour {
    fn add_segment(&mut self);
    fn movement(&mut self);
    fn collision(&mut self, food: &mut Food);
}

struct Snake {
    body: Vec<Position>,
    direction: Direction,
    points: i32,
}

impl SnakeBehaviour for Snake {
    fn add_segment(&mut self) {
        let old_head_x = self.body[0].x;
        let old_head_y = self.body[0].y;

        let new_head = match self.direction {
            Direction::Up => Position { x: old_head_x, y: old_head_y - 15 },
            Direction::Down => Position { x: old_head_x, y: old_head_y + 15 },
            Direction::Right => Position { x: old_head_x + 15, y: old_head_y },
            Direction::Left => Position { x: old_head_x - 15, y: old_head_y },
        };

        self.body.insert(0, new_head);
    }

    fn movement(&mut self) {
        self.add_segment();
        self.body.pop();
    }

    fn collision(&mut self, food: &mut Food) {

        // food collision

        let mut head_x = self.body[0].x;
        let mut head_y = self.body[0].y;
        
        let mut head = Rectangle::new(head_x as f32, head_y as f32, 15.0, 15.0);
        let food_rect = Rectangle::new(food.x as f32, food.y as f32, 15.0, 15.0);
        
        if head.check_collision_recs(&food_rect) {
            self.add_segment();
            self.points += 1;
            
            let mut rng = rand::thread_rng();
            food.x = rng.gen_range(0..785);
            food.y = rng.gen_range(0..585);
        }

        // body collision

        head_x = self.body[0].x;
        head_y = self.body[0].y;
 
        head = Rectangle::new(head_x as f32, head_y as f32, 15.0, 15.0);
 
        for segment in &self.body[1..] {
            let seg_rect = Rectangle::new(segment.x as f32, segment.y as f32, 15.0, 15.0);

            if seg_rect.check_collision_recs(&head) {
                exit(0);
            }
        }

        // wall collision

        if head_x > 800 || head_x < 0 {
            exit(0);
        }
        else if head_y > 600 || head_y < 0 {
            exit(0);
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Snake Game")
        .build();

    rl.set_target_fps(20);
    
    let mut snake = Snake {
        body: vec![Position { x: 100, y: 100 }],
        direction: Direction::Right,
        points: 0,
    };

    let mut food = Food {
        x: 130,
        y: 130,
    };

    while !rl.window_should_close() {

        // input

        if rl.is_key_down(KeyboardKey::KEY_UP) && snake.direction != Direction::Down {
            snake.direction = Direction::Up;
        }

        if rl.is_key_down(KeyboardKey::KEY_DOWN) && snake.direction != Direction::Up {
            snake.direction = Direction::Down;
        }

        if rl.is_key_down(KeyboardKey::KEY_LEFT) && snake.direction != Direction::Right {
            snake.direction = Direction::Left;
        }

        if rl.is_key_down(KeyboardKey::KEY_RIGHT) && snake.direction != Direction::Left {
            snake.direction = Direction::Right;
        }

        // logic
        
        snake.movement();
        snake.collision(&mut food);

        // draw food 

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        
        d.draw_rectangle(food.x, food.y, 15, 15, Color::DARKGREEN);

        // draw snake 
        
        for segment in &snake.body {
            d.draw_rectangle(segment.x, segment.y, 15, 15, Color::DARKRED);
        }

        // draw points
        
        d.draw_text(&format!("Points: {}", snake.points), 10, 10, 20, Color::BLACK);
    }
}
