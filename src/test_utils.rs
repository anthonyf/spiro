pub fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-4
}

macro_rules! assert_close {
    ($a:expr, $b:expr) => {{
        let sa = stringify!($a);
        let sb = stringify!($b);
        let a = $a;
        let b = $b;
        assert!($crate::test_utils::close(a, b), "{sa} = {a} != {sb} = {b}");
    }};
}

pub(crate) use assert_close;
