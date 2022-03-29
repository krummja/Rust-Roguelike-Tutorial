use rltk::{ RGB, Rltk, RandomNumberGenerator };
use std::cmp::{ max, min };

use super::constants::*;


#[derive(PartialEq, Copy, Clone)]
pub enum TileType
{
    Wall,
    Floor,
}


pub fn xy_idx(x: i32, y: i32) -> usize
{
    return (y as usize * SCREEN_WIDTH as usize) + x as usize;
}


pub fn new_map() -> Vec<TileType>
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


pub fn draw_map(map: &[TileType], ctx: &mut Rltk) -> ()
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
