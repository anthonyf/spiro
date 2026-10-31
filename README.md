# spiro

A small declarative language for defining spirographs, including fractal ones,
rendered with [macroquad](https://github.com/not-fl3/macroquad). A 3-day long
weekend project to learn Rust.

## Core idea

Every spirograph is a **ring size** plus a list of **spinning arms**. Each arm
has a length, a whole-number speed, and a phase. The arms are joined end to
end, and the pen is at the tip of the last one.

- **The curve** comes only from the arms. It closes after `2π / gcd(speeds)`.
- **The wheels** are drawn for show, and are derived from the arms. Starting
  from the ring, each wheel is centred at the end of its arm, with size
  `|arm length − previous wheel|`. A wheel sits inside the previous one when
  its arm is shorter than that wheel, and outside when it's longer, so the
  side never has to be stored.

Because the wheels are derived, the input can't describe wheels that fail to
touch. `chain` is sugar that generates arms; the renderer only sees a ring and
a list of arms per layer.

![Spirograph glossary: the toy (ring, wheel, hole, pen) next to the same
curve drawn as two spinning arms](docs/glossary.svg)

## Syntax sketch (draft)

```
canvas 800 background "#fdf6e3"

layer {
  ring 96
  arm 60 speed 3         # classic spirograph: the toy's ring 96,
  arm 28.8 speed -5      # wheel 36, hole 0.8
  pen teal 1.5
}

layer {
  chain 7 {              # fractal: 7 arms
    scale 1/3            # each a third the length...
    speed -4             # ...and 4x faster, reversed
  }
  pen crimson 0.5
}
```

### Open questions before freezing the syntax

1. **`chain`:** where does it start? It needs a first arm length, a first
   speed, and a ring.
2. **Fractions:** is `1/3` one number, or a number, a `/`, and a number?
3. **Negative numbers:** is `-4` one token, or a minus sign and `4`?
4. **Comments vs. colours:** comments start with `#`, and so does
   `"#fdf6e3"`. Are hex colours always quoted?
5. **`canvas 800`:** the window now fits the curve automatically. Is a size
   still needed, perhaps only for export?
6. **Colour names:** which names are allowed?
7. **`ring`:** is it required? What does a layer with no ring draw?
8. **Toy sugar:** add `wheel 36 hole 0.8` back later as shorthand that turns
   into arms?

## Pipeline

1. **Lexer:** text → tokens.
2. **Parser:** tokens → syntax tree (hand-written recursive descent).
3. **Evaluator:** syntax tree → layers, each a ring plus a list of arms.
4. **Renderer:** macroquad on screen, SVG/PNG export behind a trait.
5. **Hot reload:** check the file's mtime each frame and re-render on save.

## Plan

- [x] Day 1: renderer from hardcoded arms; classic and fractal curves; stop
      when the curve closes (`2π / gcd` of the speeds). Also done: animated
      drawing, spinning arms, touching wheels, and fitting the window.
- [ ] Day 2: lexer, parser, evaluator; errors with line/column
      ([`ariadne`](https://crates.io/crates/ariadne)); hot reload
- [ ] Day 3: layers, colors, `chain`, SVG/PNG export.
      Stretch: web build, `let`

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
