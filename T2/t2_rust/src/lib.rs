use wasm_bindgen::prelude::wasm_bindgen;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};


#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct AStarState {
    x: i32,
    y: i32,
    cost: i32,
    est: i32,
    first_move: i32,
}

impl Ord for AStarState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.est.cmp(&self.est).then_with(|| other.cost.cmp(&self.cost))
    }
}

impl PartialOrd for AStarState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn manhattan(x: i32, y: i32, fx: i32, fy: i32) -> i32 {
    (x - fx).abs() + (y - fy).abs()
}


fn find_dir(
    head_x: i32,
    head_y: i32,
    fruit: &[i32],
    obstacles: &[i32],
    snake: &[i32],
    max_steps: i32,
) -> i32 {

    let mut blocked = [[false; 9]; 9];

    for i in (0..obstacles.len()).step_by(2) {
        let x = obstacles[i];
        let y = obstacles[i + 1];
        if x >= 1 && x <= 8 && y >= 1 && y <= 8 {
            blocked[x as usize][y as usize] = true;
        }
    }

    if snake[2] >= 1 && snake[2] <= 8 && snake[3] >= 1 && snake[3] <= 8 {
        blocked[snake[2] as usize][snake[3] as usize] = true;
    }

    if head_x < 1 || head_x > 8 || head_y < 1 || head_y > 8 {
        return -1;
    }


    if head_x == fruit[0] && head_y == fruit[1] {
        return 0;
    }


    let head_state = AStarState {
        x: head_x,
        y: head_y,
        cost: 0,
        est: manhattan(head_x, head_y, fruit[0], fruit[1]),
        first_move: -2
    };

    let mut open_set = BinaryHeap::new();
    open_set.push(head_state);

    let mut visited: HashSet<(i32, i32, i32)> = HashSet::new();
    visited.insert((head_x, head_y, 0));

    let directions = [(0, 1, 0), (-1, 0, 1), (0, -1, 2), (1, 0, 3)];

    while let Some(state) = open_set.pop() {

        if state.x == fruit[0] && state.y == fruit[1] {

            return state.first_move;
        }
        if state.cost >= max_steps {
            continue;
        }
        for &(dx, dy, move_code) in &directions {
            let nx = state.x + dx;
            let ny = state.y + dy;
            if nx < 1 || nx > 8 || ny < 1 || ny > 8 {
                continue;
            }
            if blocked[nx as usize][ny as usize] {
                continue;
            }
            let new_cost = state.cost + 1;
            let new_step = (nx, ny, new_cost);
            if visited.contains(&new_step) {
                continue;
            }
            visited.insert(new_step);
            let new_est = new_cost + manhattan(nx, ny, fruit[0], fruit[1]);
            let new_first_move = if state.first_move == -2 {
                move_code
            } else {
                state.first_move
            };
            let next_state = AStarState {
                x: nx,
                y: ny,
                cost: new_cost,
                est: new_est,
                first_move: new_first_move,
            };
            open_set.push(next_state);
        }
    }
    -1
}
#[wasm_bindgen]
pub fn greedy_snake_move_barriers(
    body: &[i32],
    fruit: &[i32],
    obstacles: &[i32],
) -> i32 {
    let head_x = body[0];
    let head_y = body[1];
    let fruit_x = fruit[0];
    let fruit_y = fruit[1];

    find_dir(head_x, head_y, &fruit, obstacles, body, 200)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_path() {

        let snake = [2, 2, 2, 1, 2, 0, 1, 0];
        let obstacles = [0; 24];
        let first_move = find_dir(2, 2, &[7, 2], &obstacles, &snake, 50);
        assert_eq!(first_move, 3);
    }

    #[test]
    fn test_path_needs_to_turn() {

        let snake = [3, 3, 4, 3, 3, 2, 2, 1];
        let obstacles = [0; 24];
        let first_move = find_dir(3, 3, &[5, 3], &obstacles, &snake, 50);

        assert_ne!(first_move, 3);
    }

    #[test]
    fn test_no_path() {

        let snake = [3,2,3,1,4,1,5,1];

        let mut obstacles = [0; 24];
        let block_coords = [
            (1,2),(2,1)
        ];
        for (i, &(x, y)) in block_coords.iter().enumerate() {
            obstacles[i * 2] = x;
            obstacles[i * 2 + 1] = y;
        }
        let first_move = greedy_snake_move_barriers(&snake, &[1,1], &obstacles);
        assert_eq!(first_move, -1);
    }

    #[test]
    fn test_full_path() {
        let mut snake = [5,1,4,1,3,1,2,1];
        let fruit = [4,2];

        let mut obstacles = [0; 24];
        let block_coords = [
            (5,2),(6,2),(7,2)
        ];
        for (i, &(x,y)) in block_coords.iter().enumerate() {
            obstacles[i * 2] = x;
            obstacles[i * 2 + 1] = y;
        }

        let mut found = false;
        while snake[0] != fruit[0] || snake[1] != fruit[1] {
            let move_dir = greedy_snake_move_barriers(&snake, &fruit, &obstacles);
            if move_dir == -1 {
                println!("No path found!");
                break;
            } else {
                println!("move:{}", move_dir);
                snake[6] = snake[4];
                snake[7] = snake[5];
                snake[4] = snake[2];
                snake[5] = snake[3];
                snake[2] = snake[0];
                snake[3] = snake[1];
                snake[0] += match move_dir {
                    0 => 0,
                    1 => -1,
                    2 => 0,
                    3 => 1,
                    _ => unreachable!(),
                };
                snake[1] += match move_dir {
                    0 => 1,
                    1 => 0,
                    2 => -1,
                    3 => 0,
                    _ => unreachable!(),
                };
            }
        }
    }
}
