#[derive(Debug, Clone, Copy)]
struct Circle {
    radius: f32,
    phase: f32,
    speed: f32,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

fn pen_point(circles: &[Circle], time: f32) -> Point {
    let mut x = 0.0;
    let mut y = 0.0;

    for circle in circles {
        let angle = circle.phase + circle.speed * time;
        x += circle.radius * angle.cos();
        y += circle.radius * angle.sin();
    }

    Point { x, y }
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
    let pen_point = pen_point(&circles, 0.0);

    //println!("Circles: {:#?}", circles);
    println!("Pen point: {:#?}", pen_point);
}
