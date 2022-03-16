# OXUI
Cross platform native GUI in Rust

## GUI with ideas from
- Flutter
    - Widget Tree
    - Render Object
    - Skia
- Jetpack Compose
    - [compose runtime](https://github.com/cksac/compose-rt)
    - State via positional memorization
    - Incremental computation

## Status
Experimental and very early stage, it is far from usable.
- [ ] Widgets
    - [x] Flex
    - [x] Constrained Box
    - ...
- [ ] Rendering object
    - [^] RenderFlex
    - ...
- [ ] App runner    
- [ ] event handling
    - [^] hit test
    - ...
- [ ] optimization
    - [ ] repaint boundary
    - [ ] layer composition

^: partially


## Examples
- Flex layout with positional state test
    - cargo run --example run_app

