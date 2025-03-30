// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn greedy_snake_move(body: &[i32], fruit: &[i32]) -> i32 {
    let mut head_x = body[0];
    let mut head_y = body[1];
    let mut body_x = body[2];
    let mut body_y = body[3];
    let mut fruit_x = fruit[0];
    let mut fruit_y = fruit[1];

    if head_x == fruit_x {
        if head_y < fruit_y {
            if body_x == head_x && body_y > head_y {
                if head_x + 1 <= 8 {
                    return 3;
                }
                return 1;
            }
            return 0;
        } else if head_y > fruit_y {
            if body_x == head_x && body_y < head_y {
                if head_x + 1 <= 8 {
                    return 3;
                }
                return 1;
            }
            return 2;
        }
        return -1;
    } else if head_x < fruit_x {
        if head_y == fruit_y {
            if body_y == head_y && body_x > head_x {
                if head_y + 1 <= 8 {
                    return 0;
                }
                return 2;
            }
            return 3;
        } else if head_y < fruit_y {
            if body_x == head_x && body_y > head_y {
                return 3;
            }
            return 0;
        } else if head_y > fruit_y {
            if body_x == head_x && body_y < head_y {
                return 3;
            }
            return 2;
        }
        return -1;
    } else {
        if head_y == fruit_y {
            if body_y == head_y && body_x < head_x {
                if head_y + 1 <= 8 {
                    return 0;
                }
                return 2;
            }
            return 1;
        } else if head_y < fruit_y {
            if body_x == head_x && body_y > head_y {
                return 1;
            }
            return 0;
        } else if head_y > fruit_y {
            if body_x == head_x && body_y < head_y {
                return 1;
            }
            return 2;
        }
        return -1;
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn greedy_snake_move_test() {
//         assert_eq!(greedy_snake_move(&[1, 2, 3, 4], &[5, 6]), 0);
//         assert_eq!(greedy_snake_move(&[1, 2, 3, 4], &[5, 2]), 3);
//         assert_eq!(greedy_snake_move(&[1, 2, 3, 4], &[1, 6]), 2);
//         assert_eq!(greedy_snake_move(&[1, 2, 3, 4], &[1, 2]), -1);
//     }
// }
