/** Exported memory */
export declare const memory: WebAssembly.Memory;
/**
 * assembly/index/greedy_snake_move
 * @param snake `~lib/array/Array<i32>`
 * @param food `~lib/array/Array<i32>`
 * @returns `i32`
 */
export declare function greedy_snake_move(snake: Array<number>, food: Array<number>): number;
/**
 * assembly/index/greedySnakeMoveBarriers
 * @param n `i32`
 * @param snake `~lib/array/Array<i32>`
 * @param food `~lib/array/Array<i32>`
 * @param barriers `~lib/array/Array<i32>`
 * @returns `i32`
 */
export declare function greedySnakeMoveBarriers(n: number, snake: Array<number>, food: Array<number>, barriers: Array<number>): number;
/**
 * assembly/index/greedy_policy_wrapper
 * @param n `i32`
 * @param snake `~lib/array/Array<i32>`
 * @param otherSnakeCount `i32`
 * @param otherSnakes `~lib/array/Array<i32>`
 * @param food_num `i32`
 * @param foods `~lib/array/Array<i32>`
 * @param remainingRounds `i32`
 * @returns `i32`
 */
export declare function greedy_policy_wrapper(n: number, snake: Array<number>, otherSnakeCount: number, otherSnakes: Array<number>, food_num: number, foods: Array<number>, remainingRounds: number): number;
/**
 * assembly/index/greedy_snake_step
 * @param n `i32`
 * @param snake `~lib/array/Array<i32>`
 * @param otherSnakeCount `i32`
 * @param otherSnakes `~lib/array/Array<i32>`
 * @param food_num `i32`
 * @param foods `~lib/array/Array<i32>`
 * @param remainingRounds `i32`
 * @returns `i32`
 */
export declare function greedy_snake_step(n: number, snake: Array<number>, otherSnakeCount: number, otherSnakes: Array<number>, food_num: number, foods: Array<number>, remainingRounds: number): number;
