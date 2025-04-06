use std::collections::VecDeque;
use std::io::sink;
use wasm_bindgen::prelude::wasm_bindgen;
use rand::random;

struct MoveScore {
    direction: i32,
    safety_score: f64,
    free_score: f64,
    food_score: f64,
    center_score: f64,
    total_score: f64,
}


pub fn greedy_snake_step_4(
    size: i32,
    body: &[i32],
    snake_num: i32,
    other_snakes: &[i32],
    fruit_num: i32,
    fruit: &[i32],
    round: i32,
) -> i32 {
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

        let safety_score = evaluate_safety(next_x, next_y, other_snakes, size);

        let free_score = flood_fill_area(next_x, next_y, size, body, other_snakes);

        let fruit_score = evaluate_fruit(next_x, next_y, fruit);

        let center_score = evaluate_center(next_x, next_y, size);

        let safety_weight = 0.3;
        let free_weight = 0.2;
        let food_weight = 0.3;
        let center_weight = 0.2;
        let total_score =
            (safety_score * safety_weight)
                + (free_score * free_weight)
                + (fruit_score * food_weight)
                + (center_score * center_weight);

        choices.push(MoveScore {
            direction: dir,
            safety_score,
            free_score,
            food_score: fruit_score,
            center_score,
            total_score,
        });
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
    let mut min_dist = std::i32::MAX;
    for chunk in fruit.chunks(2) {
        let dx = new_x - chunk[0];
        let dy = new_y - chunk[1];
        let dist = dx.abs() + dy.abs();
        if dist < min_dist {
            min_dist = dist;
        }
    }
    if min_dist == 0 {
        1.0
    } else { 1.0 / (min_dist as f64) }
}

fn evaluate_center(new_x: i32, new_y: i32, size: i32) -> f64 {
    let d_left = (new_x - 1).abs();
    let d_right = (new_x - size).abs();
    let d_up = (new_y - 1).abs();
    let d_down = (new_y - size).abs();
    let min_dist = d_left.min(d_right).min(d_up).min(d_down);
    let max_dist = (size -1) as f64 / 2.0;
    let score = (min_dist as f64) / (max_dist as f64);
    if score > 1.0 {
        1.0
    } else {
        score
    }
}