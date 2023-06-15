use bracket_lib::prelude::*;

const NUM_AGENTS: u8 = 6;
const NUM_TILES: u8 = 20;
const NUM_HOLES: u8 = 20;
const NUM_OBSTACLES: u8 = 20;

struct State {
    agents: Vec<Agent>,
    tiles: Vec<Tile>,
    holes: Vec<Hole>,
    obstacles: Vec<Obstacle>,
}

struct Tile {
    x: u32,
    y: u32,
    score: u8,
}

impl Tile {
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(self.x, self.y, BLACK, WHITE, to_cp437('*'));
    }
}

struct Hole {
    x: u32,
    y: u32,
}

impl Hole {
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(self.x, self.y, BLACK, WHITE, to_cp437('O'));
    }
}

struct Obstacle {
    x: u32,
    y: u32,
}

impl Obstacle {
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(self.x, self.y, BLACK, WHITE, to_cp437('#'));
    }
}

struct Agent {
    x: u32,
    y: u32,
    id: u8,
}

impl Agent {
    fn update(&mut self, _ctx: &mut BTerm) {
        self.x += 1;
        self.y += 1;
    }

    fn render(&mut self, ctx: &mut BTerm) {
        let color = match self.id {
            0 => RED,
            1 => GREEN,
            2 => BLUE,
            3 => CYAN,
            4 => MAGENTA,
            5 => YELLOW,
            _ => WHITE,
        };
        ctx.set(self.x, self.y, color, WHITE, to_cp437('A'));
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.play(ctx);
    }
}

impl State {
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(WHITE);
        for agent in self.agents.iter_mut() {
            agent.render(ctx);
        }
        for tile in self.tiles.iter_mut() {
            tile.render(ctx);
        }
        for hole in self.holes.iter_mut() {
            hole.render(ctx);
        }
        for obstacle in self.obstacles.iter_mut() {
            obstacle.render(ctx);
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        for agent in self.agents.iter_mut() {
            agent.update(ctx);
        }
        self.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Tileworld")
        .with_fps_cap(30.0)
        .build()?;
    let mut rng = RandomNumberGenerator::new();
    let mut agents: Vec<Agent> = Vec::new();
    for i in 0..NUM_AGENTS as u8 {
        let a = Agent {
            x: i as u32 * 2 as u32,
            y: i as u32 * 2 as u32,
            id: i,
        };
        agents.push(a);
    }
    let mut tiles: Vec<Tile> = Vec::new();
    for _ in 0..NUM_TILES as u8 {
        let a = Tile {
            x: rng.range(0, 80),
            y: rng.range(0, 50),
            score: rng.range(1, 5),
        };
        tiles.push(a);
    }
    let mut holes: Vec<Hole> = Vec::new();
    for _ in 0..NUM_HOLES as u8 {
        let a = Hole {
            x: rng.range(0, 80),
            y: rng.range(0, 50),
        };
        holes.push(a);
    }
    let mut obstacles: Vec<Obstacle> = Vec::new();
    for _ in 0..NUM_OBSTACLES as u8 {
        let a = Obstacle {
            x: rng.range(0, 80),
            y: rng.range(0, 50),
        };
        obstacles.push(a);
    }
    let state = State {
        agents,
        tiles,
        holes,
        obstacles,
    };
    main_loop(context, state)
}
