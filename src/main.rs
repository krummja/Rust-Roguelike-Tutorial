use rltk::{ Rltk, GameState, RGB };
use rltk::RltkBuilder;
use specs::prelude::*;

mod components;
mod constants;
mod map;
mod player;
mod rect;
mod visibility_system;

pub use components::*;
pub use constants::*;
pub use map::*;
pub use player::*;
pub use visibility_system::*;


pub struct State
{
    ecs: World,
}


impl State
{
    fn run_systems(&mut self) -> ()
    {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}


impl GameState for State
{
    fn tick(&mut self, ctx: &mut Rltk)
    {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        draw_map(&self.ecs, ctx);

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

    game_state.ecs.register::<Position>();
    game_state.ecs.register::<Renderable>();
    game_state.ecs.register::<Player>();
    game_state.ecs.register::<Viewshed>();

    let map: Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    game_state.ecs.insert(map);

    game_state.ecs.create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();

    return rltk::main_loop(context, game_state);
}

