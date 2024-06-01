# Terminal Animator Engine In Rust

This is a terminal animator engine written in Rust. It provides several text effects that can be used to animate text in the terminal.

## Features

### Text Effects

1. **Scattering**: This effect makes the text appear as if it's scattering across the terminal. Implemented in [`scattering.rs`](src/text_effects/scattering.rs).

2. **Typewriter**: This effect makes the text appear as if it's being typed out in real-time. Implemented in [`typewriter.rs`](src/text_effects/typewriter.rs).

3. **Scrolling**: This effect makes the text appear as if it's scrolling across the terminal. There are two variants of this effect: ScrollingLeft and ScrollingRight. Implemented in [`scrolling.rs`](src/text_effects/scrolling.rs).

4. **Blinking**: This effect makes the text blink in the terminal. Implemented in [`blinking.rs`](src/text_effects/blinking.rs).

5. **Color Changing**: This effect changes the color of the text in the terminal. Implemented in [`color_changing.rs`](src/text_effects/color_changing.rs).

6. **Climbing**: This effect makes the text appear as if it's climbing up the terminal. Implemented in [`climbing.rs`](src/text_effects/climbing.rs).

### Animation

The [`Animation`](src/structs/animation.rs) struct is used to create an animation from a sequence of frames. Each frame consists of a string of text and a text effect. The animation plays each frame in sequence, applying the text effect to the text before displaying it in the terminal.

## Usage

To use the terminal animator engine, create an `Animation` with a sequence of `Frame`s, then call `animation.next_frame()` in a loop. Here is an example:

```rs
let frames = vec![
    Frame::new("I am in love with you please marry me!", Climbing),
    Frame::new("Alice alice alice alice alice alice", Climbing),
    Frame::new("I want to be with you forever!", Climbing),
];
let mut animation = Animation::new(frames, Duration::from_millis(100));
loop {
    if let Err(e) = animation.next_frame() {
        eprintln!("Error displaying frame: {}", e);
        break;
    }
}
