use crate::{Direction, PlacedPath};
use crate::parsing::{Game, GoldenPoint, PathType, Point};

pub fn find_optimal_paths(game: &mut Game) {
    let mut placed = vec![];

    for i in 0..1 {
        let (p1, p2) = get_closer_goldens(game);
        let mut silver = None;
        if let Some((x, y)) = does_rect_contains_silver(game, p1.x, p1.y, p2.x, p2.y) {
            silver = Some((x, y));
            while let Some((x, y)) = does_rect_contains_silver(game, p1.x, p1.y, x, y) {
                silver = Some((x, y));
            }
        }

        if let Some((x, y)) = silver {
            let paths = link_golden_points(game, &p1, &Point { x, y });
            placed.extend(paths);
        } else {
            let paths = link_golden_points(game, &p1, &p2);
            placed.extend(paths);
            game.golden_points.iter_mut().for_each(|p| {
                if p.x == p2.x && p.y == p2.y {
                    p.is_linked = true;
                }
            });
        }

        game.golden_points.iter_mut().for_each(|p| {
            if p.x == p1.x && p.y == p1.y {
                p.is_linked = true;
            }
        });
    }

    game.placed_paths = placed;
}

pub fn link_golden_points(game: &mut Game, g1: &Point, g2: &Point) -> Vec<PlacedPath> {
    let mut placed_paths = Vec::new();

    let mut last_path_x: u16 = g1.x;
    let mut last_path_y: u16 = g1.y;

    let mut delta_x: i16 = g2.x as i16 - g1.x as i16;
    let mut delta_y: i16 = g2.y as i16 - g1.y as i16;

    let mut last_out: Vec<Direction> = vec![];
    if g1.x < g2.x {
        last_out.push(Direction::Right);
    } else if g1.x > g2.x {
        last_out.push(Direction::Left);
    }
    if g1.y < g2.y {
        last_out.push(Direction::Down);
    } else if g1.y > g2.y {
        last_out.push(Direction::Up);
    }

    let mut i = 0;
    while !is_path_linked_to_golden(delta_x, delta_y, &last_out) && i < 100 {
        let res = get_cheaper_path_to_move_with_out(game, last_out, delta_x, delta_y, false);
        if res.is_none() {
            println!("No path found!");
            break;
        }
        let (id, directions, out, cost) = res.unwrap();
        last_out = directions.clone();
        last_out.retain(|x| *x != out);

        let out = out.invert();
        last_path_x = out.move_x(last_path_x);
        last_path_y = out.move_y(last_path_y);

        let placed_path = PlacedPath { id: id.clone(), x: last_path_x, y: last_path_y };
        placed_paths.push(placed_path);

        game.paths.iter_mut().for_each(|p| {
            if p.id == id {
                p.available -= 1;
            }
        });

        delta_x = g2.x as i16 - last_path_x as i16;
        delta_y = g2.y as i16 - last_path_y as i16;
        i += 1;
    }
    return placed_paths;
}

pub fn is_path_linked_to_golden(delta_x: i16, delta_y: i16, directions: &Vec<Direction>) -> bool {
    if delta_y == 0 {
        if delta_x == 1 {
            if directions.contains(&Direction::Right) {
                return true;
            }
        } else if delta_x == -1 {
            if directions.contains(&Direction::Left) {
                return true;
            }
        }
    } else if delta_x == 0 {
        if delta_y == 1 {
            if directions.contains(&Direction::Down) {
                return true;
            }
        } else if delta_y == -1 {
            if directions.contains(&Direction::Up) {
                return true;
            }
        }
    }
    return false;
}

pub fn get_cheaper_path_to_move_with_out(game: &Game, out: Vec<Direction>, delta_x: i16, delta_y: i16, use_alternative: bool) -> Option<(PathType, Vec<Direction>, Direction, u8)> {
    let directions = out.iter().map(|out_direction| {
        let out_direction = out_direction.invert();
        let mut move_directions = vec![out_direction.clone()];
        if delta_x > 1 {
            if out_direction != Direction::Right {
                move_directions.push(Direction::Right);
            }
        } else if delta_x < -1 {
            if out_direction != Direction::Left {
                move_directions.push(Direction::Left);
            }
        }
        if delta_y < -1 {
            if out_direction != Direction::Up {
                move_directions.push(Direction::Up);
            }
        } else if delta_y > 1 {
            if out_direction != Direction::Down {
                move_directions.push(Direction::Down);
            }
        }

        if use_alternative && delta_x == 0 && move_directions.len() == 1 {
            if out_direction != Direction::Right {
                move_directions.push(Direction::Right);
            }
            if out_direction != Direction::Left {
                move_directions.push(Direction::Left);
            }
        }
        if use_alternative && delta_y == 0 && move_directions.len() == 1 {
            if out_direction != Direction::Up {
                move_directions.push(Direction::Up);
            }
            if out_direction != Direction::Down {
                move_directions.push(Direction::Down);
            }
        }
        return move_directions;
    }).collect();

    let result = get_cheaper_path_to_move(&game, directions);
    return if result.is_none() && !use_alternative {
        get_cheaper_path_to_move_with_out(game, out, delta_x, delta_y, true)
    } else {
        result
    }
}


pub fn get_cheaper_path_to_move(game: &Game, directions: Vec<Vec<Direction>>) -> Option<(PathType, Vec<Direction>, Direction, u8)> {
    let mut min_cost = u8::MAX;
    let mut min_tile = None;

    for path in game.paths.iter() {
        if path.available == 0 {
            continue;
        }
        for wanted_directions in directions.iter() {
            // Path directions must contain at least the first wanted direction plus any other wanted direction
            for directions in path.id.get_connections() {
                let mut satisfied = 0;
                for direction in wanted_directions {
                    if satisfied == 0 { // First tile
                        if directions.contains(direction) {
                            satisfied = 1;
                        } else {
                            break;
                        }
                    } else { // Satisfied is 1
                        if directions.contains(direction) {
                            satisfied = 2;
                            break;
                        }
                    }
                }
                if satisfied > 1 && path.cost < min_cost {
                    min_cost = path.cost;
                    min_tile = Some((path.id.clone(), directions, wanted_directions[0].clone(), min_cost));
                }
            }
        }
    }
    return min_tile;
}

pub fn get_closer_goldens(game: &Game) -> (Point, Point) {
    let mut min_goldens: (Point, Point) = ((&game.golden_points[0]).into(), (&game.golden_points[1]).into());
    let mut min_dist = i16::MAX;

    for golden_point_1 in &game.golden_points {
        if golden_point_1.is_linked {
            continue;
        }
        for golden_point_2 in &game.golden_points {
            if !golden_point_2.is_linked && golden_point_1.x != golden_point_2.x && golden_point_1.y != golden_point_2.y {
                let dist = (golden_point_1.x as i16 - golden_point_2.x as i16).abs() + (golden_point_1.y as i16 - golden_point_2.y as i16).abs();
                if dist < min_dist {
                    min_dist = dist;
                    min_goldens = (golden_point_1.into(), golden_point_2.into());
                }
            }
        }
    }
    if min_goldens.0.x > min_goldens.1.x {
        min_goldens = (min_goldens.1, min_goldens.0);
    }
    return min_goldens;
}

pub fn get_closer_golden_from_point(game: &Game, x: u16, y: u16) -> &GoldenPoint {
    let mut min_goldens = &game.golden_points[0];
    let mut min_dist = u16::MAX;
    for golden_point in &game.golden_points {
        if !golden_point.is_linked && x != golden_point.x && y != golden_point.y {
            let dist = golden_point.distance(x, y);
            if dist < min_dist {
                min_dist = dist;
                min_goldens = golden_point;
            }
        }
    }
    return min_goldens;
}

pub fn does_rect_contains_silver(game: &Game, x1: u16, y1: u16, x2: u16, y2: u16) -> Option<(u16, u16)> {
    let mut x1 = x1;
    let mut y1 = y1;
    let mut x2 = x2;
    let mut y2 = y2;
    if x1 > x2 {
        x2 = x1;
    }
    if y1 > y2 {
        y2 = y1;
    }
    for silver_point in &game.silver_points {
        if silver_point.x >= x1 && silver_point.x <= x2 && silver_point.y >= y1 && silver_point.y <= y2 {
            return Some((silver_point.x, silver_point.y));
        }
    }
    return None;
}
