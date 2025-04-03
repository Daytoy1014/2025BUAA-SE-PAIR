async function instantiate(module, imports = {}) {
  const adaptedImports = {
    env: Object.assign(Object.create(globalThis), imports.env || {}, {
      abort(message, fileName, lineNumber, columnNumber) {
        // ~lib/builtins/abort(~lib/string/String | null?, ~lib/string/String | null?, u32?, u32?) => void
        message = __liftString(message >>> 0);
        fileName = __liftString(fileName >>> 0);
        lineNumber = lineNumber >>> 0;
        columnNumber = columnNumber >>> 0;
        (() => {
          // @external.js
          throw Error(`${message} in ${fileName}:${lineNumber}:${columnNumber}`);
        })();
      },
    }),
  };
  const { exports } = await WebAssembly.instantiate(module, adaptedImports);
  const memory = exports.memory || imports.env.memory;
  const adaptedExports = Object.setPrototypeOf({
    greedy_snake_move(snake, food) {
      // assembly/index/greedy_snake_move(~lib/array/Array<i32>, ~lib/array/Array<i32>) => i32
      snake = __retain(__lowerArray(__setU32, 8, 2, snake) || __notnull());
      food = __lowerArray(__setU32, 8, 2, food) || __notnull();
      try {
        return exports.greedy_snake_move(snake, food);
      } finally {
        __release(snake);
      }
    },
    greedySnakeMoveBarriers(n, snake, food, barriers) {
      // assembly/index/greedySnakeMoveBarriers(i32, ~lib/array/Array<i32>, ~lib/array/Array<i32>, ~lib/array/Array<i32>) => i32
      snake = __retain(__lowerArray(__setU32, 8, 2, snake) || __notnull());
      food = __retain(__lowerArray(__setU32, 8, 2, food) || __notnull());
      barriers = __lowerArray(__setU32, 8, 2, barriers) || __notnull();
      try {
        return exports.greedySnakeMoveBarriers(n, snake, food, barriers);
      } finally {
        __release(snake);
        __release(food);
      }
    },
    greedy_policy_wrapper(n, snake, otherSnakeCount, otherSnakes, food_num, foods, remainingRounds) {
      // assembly/index/greedy_policy_wrapper(i32, ~lib/array/Array<i32>, i32, ~lib/array/Array<i32>, i32, ~lib/array/Array<i32>, i32) => i32
      snake = __retain(__lowerArray(__setU32, 8, 2, snake) || __notnull());
      otherSnakes = __retain(__lowerArray(__setU32, 8, 2, otherSnakes) || __notnull());
      foods = __lowerArray(__setU32, 8, 2, foods) || __notnull();
      try {
        return exports.greedy_policy_wrapper(n, snake, otherSnakeCount, otherSnakes, food_num, foods, remainingRounds);
      } finally {
        __release(snake);
        __release(otherSnakes);
      }
    },
    greedy_snake_step(n, snake, otherSnakeCount, otherSnakes, food_num, foods, remainingRounds) {
      // assembly/index/greedy_snake_step(i32, ~lib/array/Array<i32>, i32, ~lib/array/Array<i32>, i32, ~lib/array/Array<i32>, i32) => i32
      snake = __retain(__lowerArray(__setU32, 8, 2, snake) || __notnull());
      otherSnakes = __retain(__lowerArray(__setU32, 8, 2, otherSnakes) || __notnull());
      foods = __lowerArray(__setU32, 8, 2, foods) || __notnull();
      try {
        return exports.greedy_snake_step(n, snake, otherSnakeCount, otherSnakes, food_num, foods, remainingRounds);
      } finally {
        __release(snake);
        __release(otherSnakes);
      }
    },
  }, exports);
  function __liftString(pointer) {
    if (!pointer) return null;
    const
      end = pointer + new Uint32Array(memory.buffer)[pointer - 4 >>> 2] >>> 1,
      memoryU16 = new Uint16Array(memory.buffer);
    let
      start = pointer >>> 1,
      string = "";
    while (end - start > 1024) string += String.fromCharCode(...memoryU16.subarray(start, start += 1024));
    return string + String.fromCharCode(...memoryU16.subarray(start, end));
  }
  function __lowerArray(lowerElement, id, align, values) {
    if (values == null) return 0;
    const
      length = values.length,
      buffer = exports.__pin(exports.__new(length << align, 1)) >>> 0,
      header = exports.__pin(exports.__new(16, id)) >>> 0;
    __setU32(header + 0, buffer);
    __dataview.setUint32(header + 4, buffer, true);
    __dataview.setUint32(header + 8, length << align, true);
    __dataview.setUint32(header + 12, length, true);
    for (let i = 0; i < length; ++i) lowerElement(buffer + (i << align >>> 0), values[i]);
    exports.__unpin(buffer);
    exports.__unpin(header);
    return header;
  }
  const refcounts = new Map();
  function __retain(pointer) {
    if (pointer) {
      const refcount = refcounts.get(pointer);
      if (refcount) refcounts.set(pointer, refcount + 1);
      else refcounts.set(exports.__pin(pointer), 1);
    }
    return pointer;
  }
  function __release(pointer) {
    if (pointer) {
      const refcount = refcounts.get(pointer);
      if (refcount === 1) exports.__unpin(pointer), refcounts.delete(pointer);
      else if (refcount) refcounts.set(pointer, refcount - 1);
      else throw Error(`invalid refcount '${refcount}' for reference '${pointer}'`);
    }
  }
  function __notnull() {
    throw TypeError("value must not be null");
  }
  let __dataview = new DataView(memory.buffer);
  function __setU32(pointer, value) {
    try {
      __dataview.setUint32(pointer, value, true);
    } catch {
      __dataview = new DataView(memory.buffer);
      __dataview.setUint32(pointer, value, true);
    }
  }
  return adaptedExports;
}
export const {
  memory,
  greedy_snake_move,
  greedySnakeMoveBarriers,
  greedy_policy_wrapper,
  greedy_snake_step,
} = await (async url => instantiate(
  await (async () => {
    const isNodeOrBun = typeof process != "undefined" && process.versions != null && (process.versions.node != null || process.versions.bun != null);
    if (isNodeOrBun) { return globalThis.WebAssembly.compile(await (await import("node:fs/promises")).readFile(url)); }
    else { return await globalThis.WebAssembly.compileStreaming(globalThis.fetch(url)); }
  })(), {
  }
))(new URL("release.wasm", import.meta.url));
