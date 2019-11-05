## Deps

* libclang-dev
* llvm-config (llvm)
* rustup
* `make deps`

## Develop

Compare keymap struct between objects generated from C and Rust:

```bash
arm-none-eabi-objdump -s -j .rodata ../.build/obj_massdrop_alt_houqp/keyboards/massdrop/alt/keymaps/houqp/keymap.o
arm-none-eabi-objdump -s -j .rodata.keymaps rust_keymap.o
```

## Reference

* https://forge.rust-lang.org/platform-support.html
