#[derive(Debug, Clone, Copy)]
struct Circle {
    radius: f32,
    phase: f32,
    speed: f32,
}

fn main() {
    let circles = vec![
        Circle {
            radius: 1.0,
            phase: 0.0,
            speed: 1.0,
        },
        Circle {
            radius: 2.0,
            phase: 1.0,
            speed: 2.0,
        },
        Circle {
            radius: 3.0,
            phase: 2.0,
            speed: 3.0,
        },
    ];

    println!("Circles: {:#?}", circles);
}
