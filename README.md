# spiro

A small declarative language for defining spirographs, including fractal ones,
rendered with [macroquad](https://github.com/not-fl3/macroquad). A 3-day long
weekend project to learn Rust.

## Core idea

Every spirograph is a sum of spinning circles, each with a radius, speed, and
phase. `ring`, `wheel`, and `chain` are sugar; the renderer only sees a list of
circles per layer.

## Syntax sketch

```
canvas 800 background "#fdf6e3"

layer {
  ring 96
  wheel 36 hole 0.8      # classic spirograph
  pen teal 1.5
}

layer {
  chain 7 {              # fractal: 7 wheels
    scale 1/3            # each a third the size...
    speed -4             # ...and 4x faster, reversed
  }
  pen crimson 0.5
}
```

## Pipeline

1. **Lexer:** text → tokens.
2. **Parser:** tokens → syntax tree (hand-written recursive descent).
3. **Evaluator:** syntax tree → layers of circle lists.
4. **Renderer:** macroquad on screen, SVG/PNG export behind a trait.
5. **Hot reload:** check the file's mtime each frame and re-render on save.

## Plan

- [ ] Day 1: renderer from a hardcoded circle list; classic and fractal curves;
      stop when the curve closes (gcd of teeth counts)
- [ ] Day 2: lexer, parser, evaluator; errors with line/column
      ([`ariadne`](https://crates.io/crates/ariadne)); hot reload
- [ ] Day 3: layers, colors, `chain`, SVG/PNG export.
      Stretch: animate the wheels, web build, `let`

Freeze the syntax before starting: no functions or type system.

## Prep

- Install Rust via rustup.
- Read [The Rust Book](https://doc.rust-lang.org/book/) chapters 1–10, or do
  [rustlings](https://github.com/rust-lang/rustlings).

## References

- [Coding Train #61: Fractal Spirograph](https://TheCodingTrain.com/challenges/61-fractal-spirograph)
- Frank Farris, ["Wheels on Wheels on Wheels: Surprising Symmetry"](https://pages.cs.wisc.edu/~cs812-1/farris.pdf) (*Mathematics Magazine*, 1996)
- Frank Farris, *Creating Symmetry: The Artful Mathematics of Wallpaper Patterns* (2015)
- [Wolfram Demonstrations: Wheels on Wheels on Wheels](https://demonstrations.wolfram.com/WheelsOnWheelsOnWheels/)
