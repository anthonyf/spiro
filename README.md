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
touch.

![Spirograph glossary: the toy (ring, wheel, hole, pen) next to the same
curve drawn as two spinning arms](docs/glossary.svg)

## Document model

A file is a **document**: a list of **layers**. Each layer has one **pen**
and one **shape**. A shape is one of three ways to describe arms, and each
holds only the values that are free to choose:

| Shape | Inputs | Arms it produces | Ring |
|---|---|---|---|
| `classic` | a ring and a list of arms | the arms as written | as written |
| `fractal` | first length and speed, a length multiplier, a speed multiplier, a count | each arm is the previous one times the multipliers | calculated: first length / (1 + length multiplier), so the wheels touch from outside |
| `toy` | ring and wheel tooth counts, and the hole's distance from the wheel's centre | ring − wheel at speed wheel / g, then hole at speed −(ring − wheel) / g, where g = gcd(ring, wheel) | the ring's tooth count |

The evaluator turns every shape into a ring plus a list of arms, and the
renderer only ever sees that. How the drawing is viewed (for example, showing
the arms and wheels) is a setting of the viewer, not part of the document.

## Syntax sketch (draft)

```
# The same classic curve, written two ways
layer {
  pen teal 1.5
  classic {
    ring 96
    arm 60 speed 3
    arm 28.8 speed -5
  }
}

layer {
  pen teal 1.5
  toy ring 96 wheel 36 hole 28.8
}

# A Coding Train style fractal: 7 arms
layer {
  pen crimson 0.5
  fractal {
    length 60                 # the first arm...
    speed -3
    length_multiplier 1/3     # ...each next one a third as long...
    speed_multiplier -4       # ...and 4x faster, reversed
    count 7
  }
}
```

The pen's width is in screen pixels. An arm's `phase` is optional and
defaults to 0, as in `arm 60 speed 3 phase 0.5`.

### Rules the evaluator checks

The types can't express these, so they are reported as errors:

- `toy`: the wheel must be smaller than the ring.
- `fractal`: the length multiplier can't be −1, because the ring calculation
  would divide by zero.
- Every shape needs at least one arm with a non-zero speed, or the curve has
  no period.

### Open questions before freezing the syntax

1. **Fractions:** is `1/3` one number, or a number, a `/`, and a number?
2. **Negative numbers:** is `-4` one token, or a minus sign and `4`?
3. **Comments vs. colours:** comments start with `#`. If hex colours like
   `#fdf6e3` are allowed, are they always quoted?
4. **Colour names:** which names are allowed?
5. **Background and canvas size:** the document has neither yet. Are they
   needed, perhaps only for export?
6. **`toy` hole:** a distance from the wheel's centre (`hole 28.8`, as in the
   sketch), or a fraction of the wheel (`hole 0.8`)?
7. **Line breaks:** do they matter, or can a whole layer go on one line?

## Pipeline

1. **Lexer:** text → tokens.
2. **Parser:** tokens → syntax tree (hand-written recursive descent).
3. **Evaluator:** document → layers, each a pen plus a ring and a list of
   arms. Reports the rule errors above.
4. **Renderer:** macroquad on screen, SVG/PNG export behind a trait.
5. **Hot reload:** check the file's mtime each frame and re-render on save.

## Plan

- [x] Day 1: renderer from hardcoded arms; classic and fractal curves; stop
      when the curve closes (`2π / gcd` of the speeds). Also done: animated
      drawing, spinning arms, touching wheels, and fitting the window.
- [ ] Day 2: lexer, parser, evaluator; errors with line/column
      ([`ariadne`](https://crates.io/crates/ariadne)); hot reload
- [ ] Day 3: colors, SVG/PNG export. Stretch: web build, `let`.
      (The document model with layers, pens and `fractal` is already done in
      Rust; `toy` is still to do.)
- [ ] Later: move the evaluator's checks into types, so invalid values
      can't be built. For example, `NonZeroU32` for the toy's wheel (which
      removes `WheelIsZero`), and a `Gears` type whose constructor checks
      wheel < ring. The same applies to other rules (a non-zero speed, a
      length multiplier that isn't −1). This needs modules, because a
      private field only protects a value outside its own module.

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
