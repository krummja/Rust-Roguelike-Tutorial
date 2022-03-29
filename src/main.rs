use rltk::{Rltk, GameState, RGB, VirtualKeyCode};
use rltk::RltkBuilder;
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;


const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

// MAP

#[derive(PartialEq, Copy, Clone)]
enum TileType
{
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize
{
    return (y as usize * SCREEN_WIDTH as usize) + x as usize;
}

fn new_map() -> Vec<TileType>
{
    let mut map = vec![TileType::Floor; SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize];

    for x in 0..SCREEN_WIDTH
    {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, SCREEN_HEIGHT - 1)] = TileType::Wall;
    }
    for y in 0..SCREEN_HEIGHT
    {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(SCREEN_WIDTH - 1, y)] = TileType::Wall;
    }

    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400
    {
        let x = rng.roll_dice(1, SCREEN_WIDTH -1 );
        let y = rng.roll_dice(1, SCREEN_HEIGHT - 1);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25)
        {
            map[idx] = TileType::Wall;
        }
    }

    return map;
}

fn draw_map(map: &[TileType], ctx: &mut Rltk) -> ()
{
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter()
    {
        match tile
        {
            TileType::Floor => {
                ctx.set(
                    x, y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('.')
                );
            }
            TileType::Wall => {
                ctx.set(
                  x, y,
                    RGB::from_f32(0.0, 0.5, 0.0),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('#')
                );
            }
        }

        x += 1;
        if x > SCREEN_WIDTH - 1
        {
            x = 0;
            y += 1;
        }
    }
}

// COMPONENTS

#[derive(Component)]
struct Position
{
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable
{
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component)]
struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) -> ()
{
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join()
    {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[destination_idx] != TileType::Wall
        {
            pos.x = min(SCREEN_WIDTH - 1, max(0, pos.x + delta_x));
            pos.y = min(SCREEN_HEIGHT - 1, max(0, pos.y + delta_y));
        }
    }
}

fn player_input(game_state: &mut State, ctx: &mut Rltk) -> ()
{
    match ctx.key
    {
        None => {}
        Some(key) => match key
        {
            VirtualKeyCode::Left  => try_move_player(-1,  0, &mut game_state.ecs),
            VirtualKeyCode::Right => try_move_player( 1,  0, &mut game_state.ecs),
            VirtualKeyCode::Up    => try_move_player( 0, -1, &mut game_state.ecs),
            VirtualKeyCode::Down  => try_move_player( 0,  1, &mut game_state.ecs),
            _ => {}
        }
    }
}


// CORE STATE

struct State
{
    ecs: World
}

impl GameState for State
{
    fn tick(&mut self, ctx: &mut Rltk)
    {
        ctx.cls();

        player_input(self, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        for (pos, render) in (&positions, &renderables).join()
        {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}


fn main() -> rltk::BError
{
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut game_state = State {
        ecs: World::new(),
    };

    game_state.ecs.insert(new_map());

    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Renderable>();
    game_state.ecs.register::<Player>();

    game_state.ecs.create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    return rltk::main_loop(context, game_state);
}
