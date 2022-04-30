const dylib = Deno.dlopen("./target/debug/hotkeyz.dll", {
  "foo": { parameters: [], result: "void" },
})

dylib.symbols.foo();