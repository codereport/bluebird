use std::cmp;

#[allow(unused_macros)]
#[macro_export]
macro_rules! phi1 {
    ($a:expr, $b:expr, $c:expr) => {{
        |x: i32, y: i32| {
            let a = $a;
            let b = $b;
            let c = $c;
            b(a(x, y), c(x, y))
        }
    }};
}

#[allow(dead_code)]
type BinOp = fn(i32, i32) -> i32;

nofmt::pls! {

    pub const _MAX_:  BinOp    = |x: i32, y: i32| cmp::max(x, y);
    pub const _MIN_:  BinOp    = |x: i32, y: i32| cmp::min(x, y);
    pub const _MUL_:  BinOp    = |x: i32, y: i32| x * y;
    pub const _PLUS_: BinOp    = |x: i32, y: i32| x + y;
    pub const _L_:    BinOp    = |x: i32, _: i32| x;
    pub const _R_:    BinOp    = |_: i32, y: i32| y;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phi1_test() {
        assert_eq!(phi1!(_MAX_, _PLUS_, _R_)(3, 2), 5);
        assert_eq!(phi1!(_R_, _MUL_, _R_)(3, 2), 4);
        assert_eq!(phi1!(_L_, _MUL_, _PLUS_)(3, 2), 15);
        assert_eq!(phi1!(_MUL_, _MIN_, _PLUS_)(3, 2), 5);
    }
}
