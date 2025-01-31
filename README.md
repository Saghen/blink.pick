> [!WARNING]
> Not ready for use

# Blink Picker (blink.pick)

## Design Goals

- Modular architecture, akin to telescope.nvim, available from Lua
  - Views
    - Preview
    - Input
    - List
      - Items (can have actions, editable where possible)
  - Pickers
  - Matchers
  - Sorting
- Core in rust
  - [frizbee](https://github.com/saghen/frizbee) matcher
  - Async IO
  - Ripgrep
  - Git bindings
- <10ms latency for all actions (typing, opening, etc)
- UI based on [swiper](https://github.com/abo-abo/swiper)
  - Re-use existing buffers where possible

