mod for4;

use std::char::MAX;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::io::sink;
use wasm_bindgen::prelude::wasm_bindgen;
use rand::random;

struct MoveScore {
    direction: i32,
    safety_score: f64,
    free_score: f64,
    food_score: f64,
    total_score: f64,
    danger_weight: f64,
}

#[wasm_bindgen]
pub fn greedy_snake_step(
    size: i32,
    body: &[i32],
    snake_num: i32,
    other_snakes: &[i32],
    fruit_num: i32,
    fruit: &[i32],
    round: i32,
) -> i32 {
    if size == 8 {
        return for4::greedy_snake_step_4(size, body, snake_num, other_snakes, fruit_num, fruit, round);
    }
    let mut choices: Vec<MoveScore> = Vec::new();
    for dir in 0..4 {
        let (next_x, next_y) = match dir {
            0 => (body[0], body[1] + 1),
            1 => (body[0] - 1, body[1]),
            2 => (body[0], body[1] - 1),
            3 => (body[0] + 1, body[1]),
            _ => (body[0], body[1]),
        };

        if next_x < 1 || next_y < 1 || next_x > size || next_y > size {
            continue;
        }

        if next_x == body[2] && next_y == body[3] {
            continue;
        }

        if crash_other_snake(next_x, next_y, other_snakes) {
            continue;
        }

        let mut danger_weight = 1.0;
        if next_x == 1 || next_x == size || next_y == 1 || next_y == size {
            let mut min_dist = std::i32::MAX;
            for i in (0..other_snakes.len()).step_by(2) {
                if (i+1) % 8 == 1 {
                    let dist = (next_x - other_snakes[i]).abs() + (next_y - other_snakes[i + 1]).abs();
                    if dist < min_dist {
                        min_dist = dist;
                    }
                }
            }
            if min_dist < 3 {
                danger_weight = 0.9;
            }
        }

        let safety_score = evaluate_safety(next_x, next_y, other_snakes, size);

        let free_score = flood_fill_area(next_x, next_y, size, body, other_snakes);

        let new_body = [
            next_x,
            next_y,
            body[0],
            body[1],
            body[2],
            body[3],
            body[4],
            body[5],
        ];

        let fruit_score = evaluate_fruit_race_adv(&new_body, other_snakes, fruit, 50-round, size);

        if size == 5 {
            let safety_weight = 0.05;
            let free_weight = 0.05;
            let food_weight = 0.9;
            let total_score =
                ((safety_score * safety_weight)
                    + (free_score * free_weight)
                    + (fruit_score * food_weight)) * danger_weight;

            choices.push(MoveScore {
                direction: dir,
                safety_score,
                free_score,
                food_score: fruit_score,
                total_score,
                danger_weight,
            });
        } else {
            let safety_weight = 0.1;
            let free_weight = 0.1;
            let food_weight = 0.8;
            let total_score =
                ((safety_score * safety_weight)
                    + (free_score * free_weight)
                    + (fruit_score * food_weight)) * danger_weight;

            choices.push(MoveScore {
                direction: dir,
                safety_score,
                free_score,
                food_score: fruit_score,
                total_score,
                danger_weight,
            });
        }
    }
    if choices.is_empty() {
        return (random::<i32>() % 4).abs()
    }
    choices.sort_by(|a, b| b.total_score.partial_cmp(&a.total_score).unwrap());
    choices[0].direction
}

fn crash_other_snake(next_x: i32, next_y: i32, other_snakes: &[i32]) -> bool {
    for i in (0..other_snakes.len()).step_by(2) {
        if (i+1) % 8 == 7 {
            continue;
        }
        if next_x == other_snakes[i] && next_y == other_snakes[i + 1] {
            return true;
        }
    }
    false
}

fn evaluate_safety(next_x: i32, next_y: i32, other_snakes: &[i32], size: i32) -> f64 {
    let mut min_dist = std::i32::MAX;
    for i in (0..other_snakes.len()).step_by(2) {
        if (i+1) % 8 == 1 {
            let dx = next_x - other_snakes[i];
            let dy = next_y - other_snakes[i + 1];
            let dist = dx.abs() + dy.abs();
            if dist < min_dist {
                min_dist = dist;
            }
        }
    }
    (min_dist as f64) / (2.0 * size as f64)
}

fn flood_fill_area(next_x: i32, next_y: i32, size: i32, body: &[i32], other_snakes: &[i32]) -> f64 {
    let mut area = 0;
    let mut visited = vec![vec![false; size as usize + 1]; size as usize + 1];
    let mut queue = VecDeque::new();
    queue.push_back((next_x, next_y));
    visited[next_x as usize][next_y as usize] = true;
    let directions = [(0, 1), (-1, 0), (0, -1), (1, 0)];

    while let Some((x, y)) = queue.pop_front() {
        area += 1;
        for &(dx, dy) in &directions {
            let new_x = x + dx;
            let new_y = y + dy;
            if new_x < 1 || new_x > size || new_y < 1 || new_y > size {
                continue;
            }
            if visited[new_x as usize][new_y as usize] {
                continue;
            }
            if crash_other_snake(new_x, new_y, other_snakes) {
                continue;
            }
            if new_x == body[2] && new_y == body[3] {
                continue;
            }
            visited[new_x as usize][new_y as usize] = true;
            queue.push_back((new_x, new_y));
        }
    }
    area as f64 / (size * size) as f64
}

fn evaluate_fruit(new_x: i32, new_y: i32, fruit: &[i32]) -> f64 {
    let mut total_dist = 0;
    let mut count = 0;

    for chunk in fruit.chunks(2) {
        if chunk.len() < 2 {
            continue;
        }
        let dx = new_x - chunk[0];
        let dy = new_y - chunk[1];
        total_dist += dx.abs() + dy.abs();
        count += 1;
    }

    if count == 0 {
        return 0.0;
    }

    1.0 / (total_dist as f64 + 1.0)
}

fn find_astar_path_length(
    start: (i32, i32),
    target: (i32, i32),
    body: &[i32],
    obstacles: [[bool; 9]; 9],
    max_steps: i32,
    size: i32
) -> Option<i32> {
    use std::collections::{BinaryHeap, HashSet};

    #[derive(Eq, PartialEq)]
    struct State {
        x: i32,
        y: i32,
        cost: i32,
        est: i32,
        body_x: i32,
        body_y: i32,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.est.cmp(&self.est).then_with(|| other.cost.cmp(&self.cost))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut blocked = obstacles;

    let (sx, sy) = start;
    let (tx, ty) = target;

    let mut open = BinaryHeap::new();
    open.push(State {
        x: sx,
        y: sy,
        cost: 0,
        est: (sx - tx).abs() + (sy - ty).abs(),
        body_x: body[2],
        body_y: body[3],
    });

    let mut visited = HashSet::new();
    visited.insert((sx, sy, 0));
    let dirs = [(0, 1), (-1, 0), (0, -1), (1, 0)];

    while let Some(state) = open.pop() {
        if state.x == tx && state.y == ty {
            return Some(state.cost);
        }

        if state.cost >= max_steps {
            continue;
        }

        for &(dx, dy) in &dirs {
            let nx = state.x + dx;
            let ny = state.y + dy;
            if nx < 1 || nx > 8 || ny < 1 || ny > 8 {
                continue;
            }
            if blocked[nx as usize][ny as usize] {
                continue;
            }
            if nx == state.body_x && ny == state.body_y {
                continue;
            }
            let step = (nx, ny, state.cost + 1);
            if visited.contains(&step) {
                continue;
            }
            visited.insert(step);
            open.push(State {
                x: nx,
                y: ny,
                cost: state.cost + 1,
                est: state.cost + 1 + (nx - tx).abs() + (ny - ty).abs(),
                body_x: state.x,
                body_y: state.y,
            });
        }
    }
    None
}

fn evaluate_fruit_race_adv(
    my_body: &[i32],
    enemy_body: &[i32],
    fruits: &[i32],
    max_steps: i32,
    size: i32
) -> f64 {
    let mut best_diff = 0.0;
    let mut best_my_path = std::i32::MAX;

    let mut obstacles = [[false; 9]; 9];
    let x = my_body[2] as usize;
    let y = my_body[3] as usize;
    obstacles[x][y] = true;

    if enemy_body.len() > 0 {
        for i in (0..enemy_body.len()).step_by(8) {
            let mut x = enemy_body[i + 2] as usize;
            let mut y = enemy_body[i + 3] as usize;
            obstacles[x][y] = true;
            x = enemy_body[i + 4] as usize;
            y = enemy_body[i + 5] as usize;
            obstacles[x][y] = true;
        }
    }

    for chunk in fruits.chunks(2) {
        if chunk.len() < 2 {
            continue;
        }
        let fx = chunk[0];
        let fy = chunk[1];

        let my_len = find_astar_path_length(
            (my_body[0], my_body[1]),
            (fx, fy),
            my_body,
            obstacles,
            max_steps,
            size
        ).unwrap_or(max_steps + 10);

        if enemy_body.len() > 0 {
            if my_len <= max_steps {
                let mut new_obstacles = [[false; 9]; 9];
                let mut x = my_body[0] as usize;
                let mut y = my_body[1] as usize;
                new_obstacles[x][y] = true;
                x = my_body[2] as usize;
                y = my_body[3] as usize;
                new_obstacles[x][y] = true;
                x = my_body[4] as usize;
                y = my_body[5] as usize;
                new_obstacles[x][y] = true;
                let mut min_enemy_len = i32::MAX;
                for i in (0..enemy_body.len()).step_by(8) {
                    for j in (0.. enemy_body.len()).step_by(8) {
                        if i == j {
                            x = enemy_body[i+2] as usize;
                            y = enemy_body[i+3] as usize;
                            new_obstacles[x][y] = true;
                        } else {
                            x = enemy_body[j] as usize;
                            y = enemy_body[j + 1] as usize;
                            x = enemy_body[j + 2] as usize;
                            y = enemy_body[j + 3] as usize;
                            new_obstacles[x][y] = true;
                            x = enemy_body[j + 4] as usize;
                            y = enemy_body[j + 5] as usize;
                            new_obstacles[x][y] = true;
                        }
                    }
                    obstacles[x][y] = true;
                    let enemy_len = find_astar_path_length(
                        (enemy_body[i], enemy_body[i + 1]),
                        (fx, fy),
                        enemy_body,
                        obstacles,
                        max_steps,
                        size
                    ).unwrap_or(max_steps + 10);
                    if enemy_len < min_enemy_len {
                        min_enemy_len = enemy_len;
                    }
                }
                if my_len < best_my_path && min_enemy_len as f64 - my_len as f64 >= best_diff {
                    best_my_path = my_len;
                    best_diff = (min_enemy_len - my_len) as f64;
                }
            }
        } else if my_len < best_my_path {
            best_my_path = my_len;
        }
    }
    if enemy_body.len() < 1 {
        return 1.0 / best_my_path as f64;
    } else {
        if best_diff == 0.0 {
            return 0.0;
        } else {
            return 1.0 - 1.0 / best_diff;
        }
    }
}