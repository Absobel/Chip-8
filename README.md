# INSTALLATION AND EXECUTION

Install rust and cargo then :

```
git clone <the url of the repo>
cd <the repo>
cargo run --release
```

Enjoy.

# LAUNCH OPTIONS

This is temporary because there is no menu or anything but you can modify the constants in src/launch_options.rs to change things like the rom that will be executed at launch, the number of instructions per second or the behavior of certain instructions that changed between different versions of Chip-8 etc.

# TODO

## Features
- [ ] Add sound
- [ ] Add a menu
- [ ] Add a debugger

# Implementation
- [ ] stack: Vec -> Array
- [ ] Remove unsafe
- [ ] Unit tests
