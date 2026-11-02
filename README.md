# spiro

A small declarative language for defining spirographs, including fractal ones,
rendered with [macroquad](https://github.com/not-fl3/macroquad). A 3-day long
weekend project to learn Rust.

## Running it

```bash
cargo run -- examples/coding-train.spiro
```

With no file name, it opens `examples/readme.spiro`. If a file has a mistake,
spiro prints where it is and exits, for example:

```
examples/broken.spiro: line 2, column 7: expected color, found 'teal'
```

To save the finished drawing as an SVG image instead of opening a window:

```bash
cargo run --release -- examples/dense-toy.spiro --svg dense-toy.svg --size 800
```

`--size` is the image's width and height in pixels (default 800). The SVG
has the curves only, without the ring, wheels and arms. To regenerate the
gallery below:

```bash
for f in examples/*.spiro; do cargo run --release -q -- "$f" --svg "docs/gallery/$(basename "$f" .spiro).svg"; done
```

### Gallery

Every file in `examples/`, exported with `--svg`. Click a picture to see its
source in [EXAMPLES.md](EXAMPLES.md).

<table>
  <tr>
    <td align="center" width="25%"><a href="EXAMPLES.md#dense-toy"><img src="docs/gallery/dense-toy.svg" width="180" alt="52-tooth wheel in a 105-tooth ring: 105 loops"></a><br><code>dense-toy.spiro</code><br>52-tooth wheel in a 105-tooth ring: 105 loops</td>
    <td align="center" width="25%"><a href="EXAMPLES.md#fractal-7-fold"><img src="docs/gallery/fractal-7-fold.svg" width="180" alt="Fractal, speed multiplier −6"></a><br><code>fractal-7-fold.spiro</code><br>Fractal, speed multiplier −6</td>
    <td align="center" width="25%"><a href="EXAMPLES.md#farris-18-fold"><img src="docs/gallery/farris-18-fold.svg" width="180" alt="Mandala from speeds 1, 19, −35"></a><br><code>farris-18-fold.spiro</code><br>Mandala from speeds 1, 19, −35</td>
    <td align="center" width="25%"><a href="EXAMPLES.md#mandala-24-fold"><img src="docs/gallery/mandala-24-fold.svg" width="180" alt="Mandala from speeds 7, −17, 31"></a><br><code>mandala-24-fold.spiro</code><br>Mandala from speeds 7, −17, 31</td>
  </tr>
  <tr>
    <td align="center" width="25%"><a href="EXAMPLES.md#coding-train"><img src="docs/gallery/coding-train.svg" width="180" alt="The Coding Train's fractal spirograph"></a><br><code>coding-train.spiro</code><br>The Coding Train's fractal spirograph</td>
    <td align="center" width="25%"><a href="EXAMPLES.md#fractal-8-fold"><img src="docs/gallery/fractal-8-fold.svg" width="180" alt="Fractal, speed multiplier −7"></a><br><code>fractal-8-fold.spiro</code><br>Fractal, speed multiplier −7</td>
    <td align="center" width="25%"><a href="EXAMPLES.md#fractal-same-direction"><img src="docs/gallery/fractal-same-direction.svg" width="180" alt="Fractal with every arm spinning the same way"></a><br><code>fractal-same-direction.spiro</code><br>Fractal with every arm spinning the same way</td>
    <td align="center" width="25%"><a href="EXAMPLES.md#string-art"><img src="docs/gallery/string-art.svg" width="180" alt="35-tooth wheel in a 96-tooth ring: 96 loops"></a><br><code>string-art.spiro</code><br>35-tooth wheel in a 96-tooth ring: 96 loops</td>
  </tr>
  <tr>
    <td align="center" width="25%"><a href="EXAMPLES.md#fractal-4-fold"><img src="docs/gallery/fractal-4-fold.svg" width="180" alt="Fractal, speed multiplier −3"></a><br><code>fractal-4-fold.spiro</code><br>Fractal, speed multiplier −3</td>
    <td align="center" width="25%"><a href="EXAMPLES.md#fractal-6-fold"><img src="docs/gallery/fractal-6-fold.svg" width="180" alt="Fractal, speed multiplier −5"></a><br><code>fractal-6-fold.spiro</code><br>Fractal, speed multiplier −5</td>
    <td align="center" width="25%"><a href="EXAMPLES.md#farris-6-fold"><img src="docs/gallery/farris-6-fold.svg" width="180" alt="Farris's wheels on wheels: speeds 1, 7, −17"></a><br><code>farris-6-fold.spiro</code><br>Farris's wheels on wheels: speeds 1, 7, −17</td>
    <td align="center" width="25%"><a href="EXAMPLES.md#farris-5-fold"><img src="docs/gallery/farris-5-fold.svg" width="180" alt="Farris-style: speeds 1, 6, −14"></a><br><code>farris-5-fold.spiro</code><br>Farris-style: speeds 1, 6, −14</td>
  </tr>
  <tr>
    <td align="center" width="25%"><a href="EXAMPLES.md#rosette"><img src="docs/gallery/rosette.svg" width="180" alt="Three toy wheels in one ring"></a><br><code>rosette.spiro</code><br>Three toy wheels in one ring</td>
    <td align="center" width="25%"><a href="EXAMPLES.md#cusps"><img src="docs/gallery/cusps.svg" width="180" alt="Deltoid, astroid and ellipse"></a><br><code>cusps.spiro</code><br>Deltoid, astroid and ellipse</td>
    <td align="center" width="25%"><a href="EXAMPLES.md#epicycloids"><img src="docs/gallery/epicycloids.svg" width="180" alt="Cardioid and nephroid"></a><br><code>epicycloids.spiro</code><br>Cardioid and nephroid</td>
    <td align="center" width="25%"><a href="EXAMPLES.md#star"><img src="docs/gallery/star.svg" width="180" alt="Five-pointed star from two arms"></a><br><code>star.spiro</code><br>Five-pointed star from two arms</td>
  </tr>
  <tr>
    <td align="center" width="25%"><a href="EXAMPLES.md#toy"><img src="docs/gallery/toy.svg" width="180" alt="Two toy wheels in a 96-tooth ring"></a><br><code>toy.spiro</code><br>Two toy wheels in a 96-tooth ring</td>
    <td align="center" width="25%"><a href="EXAMPLES.md#fractal"><img src="docs/gallery/fractal.svg" width="180" alt="The README's fractal"></a><br><code>fractal.spiro</code><br>The README's fractal</td>
    <td align="center" width="25%"><a href="EXAMPLES.md#readme"><img src="docs/gallery/readme.svg" width="180" alt="The README example: classic, toy and fractal"></a><br><code>readme.spiro</code><br>The README example: classic, toy and fractal</td>
  </tr>
</table>

A fractal or Farris curve has `m`-fold symmetry when all its speeds leave the
same remainder when divided by `m`. For a fractal whose speed multiplier is
`−k`, that gives `k + 1`-fold symmetry.

A toy curve takes longest to close when the ring and wheel tooth counts share
no factor: the wheel then goes all the way round the ring `wheel` times and
draws `ring` loops. The densest examples draw tens of thousands of line
segments per frame, so they run more smoothly with `cargo run --release`.

## Core idea

Every spirograph is a list of **spinning arms**. Each arm has a length, a
whole-number speed, and a phase. The arms are joined end to end, and the pen
is at the tip of the last one.

- **The curve** comes only from the arms. It closes after `2π / gcd(speeds)`.
- **The gear train** is worked out from the arms, and shown in the debug
  view. Every arm but the last carries a **wheel** (red) at its end. The
  wheel rolls on a **gear** (blue) at the start of its arm, which is fixed to
  the arm before. The first gear is the **ring**, which stays still. The next
  arm is a **spoke** of the wheel, so it turns with it. Turning the first arm
  drives all the rest, as in a toy.
- **The sizes** come from rolling without skidding. Seen from its gear, a
  wheel's centre goes round at `turn` (its arm's speed minus the gear's), and
  the wheel spins at `spin` (the next arm's speed minus the gear's). Then the
  wheel's radius is `|length × turn / spin|` and the gear's is
  `|length × (spin − turn) / spin|`. Depending on the speeds, the wheel sits
  outside its gear, inside it (the gear is then a ring), or around it.

For the toy, arms of 60 at speed 3 and 28.8 at speed −5 give back the
36 wheel in the 96 ring. Because the ring and wheels are worked out, the input
can't describe gears that skid or don't touch. The one impossible case is
`spin = 0`, which would need an infinitely big wheel (see the rules below).

![Spirograph glossary: the toy (ring, wheel, hole, pen) next to the same
curve drawn as two spinning arms](docs/glossary.svg)

## Document model

A file is a **document**: a list of **layers**. Each layer has one **pen**
and one **shape**. A shape is one of three ways to describe arms, and each
holds only the values that are free to choose:

| Shape | Inputs | Arms it produces | Ring |
|---|---|---|---|
| `classic` | a list of arms | the arms as written | worked out from the arms |
| `fractal` | first length and speed, a length multiplier, a speed multiplier, a count | each arm is the previous one times the multipliers | worked out from the arms |
| `toy` | ring and wheel tooth counts, and the hole's distance from the wheel's centre | ring − wheel at speed wheel / g, then hole at speed −(ring − wheel) / g, where g = gcd(ring, wheel) | the ring's tooth count, which the arms also give |

The evaluator turns every shape into a list of arms and the gear train
worked out from them, and the renderer only ever sees that. How the drawing is viewed (for example, showing
the arms and wheels) is a setting of the viewer, not part of the document.

## Syntax

```
background white

# The same classic curve, written two ways
layer {
  pen x008080 3               # teal, as a hex color
  classic {
    arm 60 speed 3
    arm 28.8 speed -5
  }
}

layer {
  pen x008080 3
  toy ring 96 wheel 36 hole 28.8
}

# A Coding Train style fractal: 7 arms
layer {
  pen red 2
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

### Grammar

```
document  = [ "background" color ] { layer }
layer     = "layer" "{" pen shape "}"
pen       = "pen" color decimal
shape     = classic | toy | fractal
classic   = "classic" "{" { arm } "}"
arm       = "arm" decimal "speed" whole [ "phase" decimal ]
toy       = "toy" "ring" unsigned "wheel" unsigned "hole" decimal
fractal   = "fractal" "{" "length" decimal  "speed" whole
                          "length_multiplier" decimal  "speed_multiplier" whole
                          "count" unsigned "}"
color     = WORD                      (a color name, or x + 6 hex digits)
decimal   = INTEGER | FLOAT | FRACTION
whole     = INTEGER
unsigned  = INTEGER that isn't negative
```

`"text"` is a word, `[ … ]` is optional, `{ … }` repeats, and `|` means one
of. A `fractal`'s settings must come in the order shown.

### Rules the evaluator checks

The types can't express these, so they are reported as errors:

- `toy`: the wheel can't be zero, and must be smaller than the ring.
  (Checked: `WheelIsZero` and `WheelTooBig`.)
- Every shape needs at least one arm, and at least one arm needs a speed
  other than 0, or the curve has no period. (Checked: `NoArms` and
  `NoSpeed`.)
- Two arms with one arm between them can't have the same speed, and arm 2
  can't have speed 0 (the ring's speed). Either way, the wheel on the middle
  arm would have to be infinitely big. For a fractal with 3 or more arms,
  this rules out a speed multiplier of 1 or −1. (Checked: `SameSpeed`.)

### Syntax decisions

- **Whitespace and line breaks** don't matter. `{ }` mark the blocks, so a
  whole layer can go on one line.
- **Comments** start with `#` and run to the end of the line.
- **Negative numbers:** a `-` directly followed by a digit is part of the
  number, so `-4` is one token. The language has no subtraction.
- **Fractions:** `1/3` is one token, with no spaces inside it, the same as
  `-4`. The numerator can be negative (`-1/3`); the denominator can't, and
  can't be zero. The parser accepts a fraction wherever it expects a decimal
  number.
- **Whole numbers:** speeds, tooth counts and `count` must be whole numbers,
  so `1.5` or `1/3` there is an error. Tooth counts and `count` can't be
  negative.
- **Colors** are either a name or a hex value:
  - names (all of macroquad's color constants): `beige`, `black`, `blue`,
    `brown`, `darkblue`, `darkbrown`, `darkgray`, `darkgreen`,
    `darkpurple`, `gold`, `gray`, `green`, `lightgray`, `lime`, `magenta`,
    `maroon`, `orange`, `pink`, `purple`, `red`, `skyblue`, `violet`,
    `white`, `yellow`;
  - hex: `x` followed by exactly 6 hex digits, `RRGGBB`, in upper or lower
    case, as in `x008080`. The lexer reads it as an ordinary word, and the
    parser treats it as a hex color wherever a color is expected. Using
    `x` instead of `#` avoids a clash with comments.
- **Background:** an optional `background <color>` line at the top of the
  file. The default is `white`.
- **Canvas size** isn't part of the file. On screen, the drawing fits the
  window; for SVG export, `--size` sets the image size.
- **`toy` hole** is a distance from the wheel's centre, as in
  `hole 28.8`.

## Pipeline

1. **Lexer:** text → tokens.
2. **Parser:** tokens → syntax tree (hand-written recursive descent).
3. **Evaluator:** document → layers, each a pen plus a list of arms and
   their gear train. Reports the rule errors above.
4. **Renderer:** macroquad on screen, or an SVG file.
5. **Hot reload:** check the file's mtime each frame and re-render on save.

## Code layout

- `src/spirograph.rs`: the maths. Arms, spirographs (the gear train,
  period and reach), pen positions, and the builders that turn `classic`,
  `fractal` and `toy` inputs into arms. A `Spirograph` can only be built by
  `Spirograph::new`, which checks the rules above and returns a
  `SpirographError` if one fails.
- `src/document.rs`: the document model (documents, layers, pens, shapes) and
  evaluating a document into spirographs, plus example documents.
- `src/lexer.rs`: text → tokens, each with a line and column.
- `src/parser.rs`: tokens → document, by recursive descent with one function
  per grammar rule. `parse_source` runs the lexer and the parser together.
- `src/error.rs`: `SpiroError`, which holds an error from any step (reading
  the file, lexing, parsing, or evaluating), so `?` works across them.
- `src/render.rs`: drawing a spirograph with macroquad, and the display mode
  (whether to show the gear train and arms).
- `src/svg.rs`: writes the finished curves as an SVG image, fitted the same
  way as the window.
- `src/main.rs`: reads the command-line options, loads the `.spiro` file and
  evaluates it once, then either writes an SVG or runs the window loop and
  the camera.
- `src/test_utils.rs`: test-only helpers, including `assert_close!`.
- `examples/`: example `.spiro` files. A test checks that every one of them
  parses and evaluates.
- `docs/gallery/`: an SVG of each example, shown in the gallery above.
- `EXAMPLES.md`: each example's picture next to its full source.

## Plan

- [x] Day 1: renderer from hardcoded arms; classic and fractal curves; stop
      when the curve closes (`2π / gcd` of the speeds). Also done: animated
      drawing, spinning arms, touching wheels, and fitting the window.
- [ ] Day 2: lexer, parser, evaluator; errors with line/column
      ([`ariadne`](https://crates.io/crates/ariadne)); hot reload.
      (Done: the lexer, parser and evaluator, loading a file from the
      command line, and one-line error messages with line and column. Still
      to do: ariadne and hot reload.)
- [ ] Day 3: colors, SVG/PNG export. Stretch: web build, `let`.
      (Done: named and hex colors, layers and pens, all three shapes, and
      SVG export with `--svg`. Not done: PNG export and the stretch goals.)
- [ ] Later: move the evaluator's checks into types, so invalid values
      can't be built. `Spirograph` already works this way: its fields are
      private, so outside its module the only way to make one is
      `Spirograph::new`. The toy could do the same, for example with
      `NonZeroU32` for its wheel (which removes `WheelIsZero`), and a
      `Gears` type whose constructor checks wheel < ring.

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
