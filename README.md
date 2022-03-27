# OXUI
Cross platform native GUI in Rust

## GUI with ideas from
- Flutter
    - Widget Tree
    - Render Object
    - Skia
- Jetpack Compose    
    - Positional memorization runtime
        - [compose-rt](https://github.com/cksac/compose-rt) written in Rust
    - Incremental computation

## Status
- Experimental and very early stage, it is far from usable.
- Contribution is welcome if you also intrested in building GUI framework :)

- [ ] Widgets
    - [x] Flex
    - [x] Constrained Box
    - [ ] ...
- [ ] Rendering object
    - [x] RenderFlex
    - [ ] ...
- [ ] App runner    
- [ ] event handling
    - [x] hit test
    - [ ] ...
- [ ] optimization
    - [ ] repaint boundary
    - [ ] layer composition


## Examples
- Nested Flex layout with positional state test
    - cargo run --example run_app
    
        https://user-images.githubusercontent.com/147393/158579311-0ac253f7-5cfc-464d-93d6-66e66dd288a0.mov


