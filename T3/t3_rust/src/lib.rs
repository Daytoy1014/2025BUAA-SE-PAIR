use wasm_bindgen::prelude::wasm_bindgen;


struct MoveScore {
    direction: i32,
    safety_score: f64,
    free_score: f64,
    food_score: f64,
    total_score: f64,
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

        let safety_score = evaluate_safety(next_x, next_y, other_snakes);
    }
    1
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

fn evaluate_safety(next_x: i32, next_y: i32, other_snakes: &[i32]) -> f64 {
    let mut min_dist = std::i32::MAX;
    for i in (0..other_snakes.len()).step_by(2) {
        if (i+1) % 8 == 1 {
            continue;
        }
        let dx = next_x - other_snakes[i];
        let dy = next_y - other_snakes[i + 1];
        let dist = dx.abs() + dy.abs();
        if dist < min_dist {
            min_dist = dist;
        }
    }
    min_dist as f64
}