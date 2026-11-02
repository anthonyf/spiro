mod document;
mod error;
mod lexer;
mod parser;
mod render;
mod spirograph;
mod svg;

#[cfg(test)]
mod test_utils;

use document::Document;
use error::SpiroError;
use macroquad::prelude::*;
use render::{DisplayMode, draw_spirograph, scale_to_fit};
use spirograph::{Spirograph, max_reach};

const DEFAULT_FILE: &str = "examples/readme.spiro";
const DEFAULT_SVG_SIZE: f32 = 800.0;

const USAGE: &str = "usage: spiro [FILE.spiro] [--svg OUT.svg] [--size PIXELS]

  With no options, opens FILE in a window (default: examples/readme.spiro).
  --svg OUT.svg   write the finished drawing to an SVG file instead
  --size PIXELS   the SVG's width and height (default: 800)";

/// What the command line asked for.
struct Options {
    file: String,
    svg: Option<String>,
    size: f32,
}

fn parse_options(args: impl Iterator<Item = String>) -> Result<Options, String> {
    let mut file = None;
    let mut svg = None;
    let mut size = DEFAULT_SVG_SIZE;
    let mut args = args;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--svg" => svg = Some(args.next().ok_or("--svg needs a file name")?),
            "--size" => {
                let value = args.next().ok_or("--size needs a number")?;
                size = value
                    .parse()
                    .ok()
                    .filter(|size: &f32| *size > 0.0)
                    .ok_or(format!("--size: '{value}' isn't a positive number"))?;
            }
            "-h" | "--help" => return Err(USAGE.to_string()),
            _ if arg.starts_with('-') => return Err(format!("unknown option '{arg}'")),
            _ if file.is_none() => file = Some(arg),
            _ => return Err(format!("unexpected argument '{arg}'")),
        }
    }
    Ok(Options {
        file: file.unwrap_or_else(|| DEFAULT_FILE.to_string()),
        svg,
        size,
    })
}

/// Reads, parses and evaluates a `.spiro` file.
fn load(path: &str) -> Result<(Document, Vec<Spirograph>), SpiroError> {
    let source = std::fs::read_to_string(path)?;
    let document = parser::parse_source(&source)?;
    let spirographs = document.spirographs()?;
    Ok((document, spirographs))
}

fn main() {
    let options = match parse_options(std::env::args().skip(1)) {
        Ok(options) => options,
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(2);
        }
    };

    let (doc, spiros) = match load(&options.file) {
        Ok(loaded) => loaded,
        Err(e) => {
            eprintln!("{}: {e}", options.file);
            std::process::exit(1);
        }
    };

    match options.svg {
        Some(out) => {
            if let Err(e) = std::fs::write(&out, svg::to_svg(&doc, &spiros, options.size)) {
                eprintln!("{out}: can't write the file: {e}");
                std::process::exit(1);
            }
        }
        None => macroquad::Window::new("spirograph", show(doc, spiros)),
    }
}

/// Opens a window and animates the drawing.
async fn show(doc: Document, spiros: Vec<Spirograph>) {
    let reach = max_reach(&spiros);

    let mut time = 0.0;

    loop {
        time += get_frame_time();

        let scale = scale_to_fit(reach, screen_width(), screen_height());

        let camera = Camera2D {
            zoom: vec2(scale / screen_width() * 2.0, scale / screen_height() * 2.0),
            target: vec2(0.0, 0.0),
            ..Default::default()
        };

        set_camera(&camera);

        clear_background(doc.background_color);

        for (spiro, layer) in spiros.iter().zip(doc.layers.iter()) {
            draw_spirograph(spiro, &layer.pen, DisplayMode::Debug, time, scale);
        }

        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn options(args: &[&str]) -> Result<Options, String> {
        parse_options(args.iter().map(|arg| arg.to_string()))
    }

    #[test]
    fn test_options_default() {
        let options = options(&[]).unwrap();
        assert_eq!(options.file, DEFAULT_FILE);
        assert_eq!(options.svg, None);
        assert_eq!(options.size, DEFAULT_SVG_SIZE);
    }

    #[test]
    fn test_options_svg_and_size() {
        let options = options(&["a.spiro", "--svg", "a.svg", "--size", "400"]).unwrap();
        assert_eq!(options.file, "a.spiro");
        assert_eq!(options.svg.as_deref(), Some("a.svg"));
        assert_eq!(options.size, 400.0);
    }

    #[test]
    fn test_options_errors() {
        assert!(options(&["--svg"]).is_err());
        assert!(options(&["--size", "big"]).is_err());
        assert!(options(&["--size", "-5"]).is_err());
        assert!(options(&["--colour"]).is_err());
        assert!(options(&["a.spiro", "b.spiro"]).is_err());
    }
}
