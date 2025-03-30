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

#[cfg(test)]
mod tests {
    use crate::greedy_snake_move;
    use rand::{Rng};

    fn generate_test_case() -> ([i32; 8], [i32; 2]) {
        let mut rng = rand::thread_rng();

        let mut fruit = [0; 2];
        fruit[0] = rng.gen_range(1..=8);
        fruit[1] = rng.gen_range(1..=8);

        let mut body = [0; 8];
        loop {
            body[0] = rng.gen_range(1..=8);
            body[1] = rng.gen_range(1..=8);

            if !(body[0] == fruit[0] && body[1] == fruit[1]) {
                break;
            }
        }

        for i in [2, 4, 6] {
            let mut dir = Vec::new();
            let mut dir2 = Vec::new();

            if body[i - 1] + 1 <= 8 && (body[i - 2] != fruit[0] || body[i - 1] + 1 != fruit[1]) {
                dir.push(0);
            }
            if body[i - 2] - 1 >= 1 && (body[i - 2] - 1 != fruit[0] || body[i - 1] != fruit[1]) {
                dir.push(1);
            }
            if body[i - 1] - 1 >= 1 && (body[i - 2] != fruit[0] || body[i - 1] - 1 != fruit[1]) {
                dir.push(2);
            }
            if body[i - 2] + 1 <= 8 && (body[i - 2] + 1 != fruit[0] || body[i - 1] != fruit[1]) {
                dir.push(3);
            }

            for k in 0..dir.len() {
                let mut j = 0;
                let mut flag = true;
                while j < i {
                    if dir[k] == 0 {
                        if body[i - 2] == body[j] && body[i - 1] + 1 == body[j + 1] {
                            flag = false;
                            break;
                        }
                    } else if dir[k] == 1 {
                        if body[i - 2] - 1 == body[j] && body[i - 1] == body[j + 1] {
                            flag = false;
                            break;
                        }
                    } else if dir[k] == 2 {
                        if body[i - 2] == body[j] && body[i - 1] - 1 == body[j + 1] {
                            flag = false;
                            break;
                        }
                    } else if dir[k] == 3 {
                        if body[i - 2] + 1 == body[j] && body[i - 1] == body[j + 1] {
                            flag = false;
                            break;
                        }
                    }
                    j += 2;
                }
                if flag {
                    dir2.push(dir[k]);
                }
            }
            let next_dir = dir2[rng.gen_range(0..dir2.len())];
            if next_dir == 0 {
                body[i] = body[i - 2];
                body[i + 1] = body[i - 1] + 1;
            } else if next_dir == 1 {
                body[i] = body[i - 2] - 1;
                body[i + 1] = body[i - 1];
            } else if next_dir == 2 {
                body[i] = body[i - 2];
                body[i + 1] = body[i - 1] - 1;
            } else if next_dir == 3 {
                body[i] = body[i - 2] + 1;
                body[i + 1] = body[i - 1];
            }
            dir.clear();
            dir2.clear();
        }

        (body, fruit)
    }

    fn check(body: &[i32; 8], fruit: &[i32; 2], dir: i32) -> bool {
        let mut head_x = body[0];
        let mut head_y = body[1];
        if dir == 0 {
            head_y += 1;
            if head_y > 8 {
                return false;
            }
            if head_x == body[0] && head_y == body[1] {
                return false;
            }
            if head_x == body[2] && head_y == body[3] {
                return false;
            }
            if head_x == body[4] && head_y == body[5] {
                return false;
            }

            return true;
        } else if dir == 1 {
            head_x -= 1;
            if head_x < 1 {
                return false;
            }
            if head_x == body[0] && head_y == body[1] {
                return false;
            }
            if head_x == body[2] && head_y == body[3] {
                return false;
            }
            if head_x == body[4] && head_y == body[5] {
                return false;
            }
            return true;
        } else if dir == 2 {
            head_y -= 1;
            if head_y < 1 {
                return false;
            }
            if head_x == body[0] && head_y == body[1] {
                return false;
            }
            if head_x == body[2] && head_y == body[3] {
                return false;
            }
            if head_x == body[4] && head_y == body[5] {
                return false;
            }
            return true;
        } else if dir == 3 {
            head_x += 1;
            if head_x > 8 {
                return false;
            }
            if head_x == body[0] && head_y == body[1] {
                return false;
            }
            if head_x == body[2] && head_y == body[3] {
                return false;
            }
            if head_x == body[4] && head_y == body[5] {
                return false;
            }
            return true;
        } else {
            println!("Invalid direction: {}", dir);
            return false;
        }
    }

    #[test]
    fn main() {
        for i in 0..100 {
            println!("test case {}", i);
            println!("========================");
            let (body, fruit) = generate_test_case();
            println!("body: {:?}, fruit: {:?}", body, fruit);
            let direction = greedy_snake_move(&body, &fruit);
            println!("Direction: {}", direction);
            assert_eq!(check(&body, &fruit, direction), true, "Invalid move");
            println!();
        }
    }
}
