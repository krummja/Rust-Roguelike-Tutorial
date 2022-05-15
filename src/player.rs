use rltk::{ VirtualKeyCode, Rltk };
use specs::prelude::*;
use super::{ Position, Player, TileType, State, Map, Viewshed };
use std::cmp::{ min, max };

use super::constants::*;


pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) -> ()
{
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join()
    {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx] != TileType::Wall
        {
            pos.x = min(SCREEN_WIDTH - 1, max(0, pos.x + delta_x));
            pos.y = min(SCREEN_HEIGHT - 1, max(0, pos.y + delta_y));

            viewshed.dirty = true;
        }
    }
}


pub fn player_input(game_state: &mut State, ctx: &mut Rltk) -> ()
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
