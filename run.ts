// deno run --allow-ffi --unstable run.ts
const library = Deno.dlopen("./target/debug/hotkeyz.dll", {
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
})

library.symbols.mouse_move_to(0, 0);
let enc = new TextEncoder();

let ret = await library.symbols.hotkey_register(enc.encode("<ctrl+y>\0"));
console.log(ret);

let i = 0;
while (i++ < 1) {
  let id = await library.symbols.hotkey_wait();
  console.log(id);
}

await library.symbols.hotkey_unregister(ret);
