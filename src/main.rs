use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;
const GRAVITY: f32 = 0.2;
const TERMINAL_VELOCITY: f32 = 2.0;
const PLAYER_START_X: i32 = 0;
const PLAYER_START_Y: i32 = 25;
const PLAYER_RENDER_X: i32 = 5;

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(PLAYER_RENDER_X, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < TERMINAL_VELOCITY {
            self.velocity += GRAVITY;
        }

        self.x += 1;
        self.y += self.velocity as i32;
    

        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -TERMINAL_VELOCITY;
    }
}

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        Self {
            player: Player::new(PLAYER_START_X, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

impl State {
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");

        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(PLAYER_START_X, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(15, "Welcome to Flappy Dragon!");
        ctx.print_centered(17, "Press [Enter] to start!");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Return => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(15, "You died!");
        ctx.print_centered(17, "Press [Enter] to restart!");
        ctx.print_centered(19, "Press [Q] to quit!");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Return => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    gap_size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut rng = RandomNumberGenerator::new();

        Self {
            x,
            gap_y: rng.range(10, SCREEN_HEIGHT - 10),
            gap_size: i32::max(2, 20 - score)
        }
    }
    
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
