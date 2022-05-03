// deno run --allow-ffi --unstable run.ts

const dll_path = "./target/debug/hotkeyz.dll";
const library = Deno.dlopen(dll_path, {
  kb_input: {
    parameters: ["pointer"],
    result: "i32",
  },
  kb_wait_keys_up: {
    parameters: ["pointer"],
    result: "i32",
    nonblocking: true,
  },
  hotkey_register: {
    parameters: ["pointer"],
    result: "i32",
    nonblocking: true,
  },
  hotkey_unregister: {
    parameters: ["i32"],
    result: "void",
    nonblocking: true,
  },
  hotkey_wait: {
    parameters: [],
    result: "i32",
    nonblocking: true,
  },
  mouse_move_to: {
    parameters: ["i32", "i32"],
    result: "i32",
  },
  mouse_move_delta: {
    parameters: ["i32", "i32"],
    result: "i32",
  },
  mouse_left_click: {
    parameters: [],
    result: "i32",
  },
  mouse_left_down: {
    parameters: [],
    result: "i32",
  },
  mouse_left_up: {
    parameters: [],
    result: "i32",
  },
  mouse_right_click: {
    parameters: [],
    result: "i32",
  },
  mouse_middle_click: {
    parameters: [],
    result: "i32",
  },
  mouse_wheel: {
    parameters: ["i32"],
    result: "i32",
  },
  mouse_hwheel: {
    parameters: ["i32"],
    result: "i32",
  },
  mouse_button_press: {
    parameters: ["i32", "i32"],
    result: "i32",
  },
  window_find: {
    parameters: ["pointer", "pointer"],
    result: "isize",
  },
  window_get_rect: {
    parameters: ["isize", "pointer", "pointer", "pointer", "pointer"],
    result: "i32",
  }
});

const enc = new TextEncoder();
function str(s: string) {
  return enc.encode(s + "\0");
}

// keyboard example
library.symbols.kb_input(str("abc"));

// // mouse example
// library.symbols.mouse_move_to(0, 0);

// // hotkey example
// const hotkey_id = await library.symbols.hotkey_register(str("<ctrl+y>"));
// console.log(ret);
// let i = 0;
// while (i++ < 1) {
//   const id = await library.symbols.hotkey_wait();
//   console.log(id);
//   if (id == hotkey_id) {
//     console.log('hello');
//   }
// }
// await library.symbols.hotkey_unregister(hotkey_id);

