fn fn_fibo(x: f32) -> f32 {
    let mut n = x;
    let mut n1 = 0.0;
    let mut n2 = 1.0;
    let k = 0;

    if n / 1.0 == x {
        for (k = 2; k <= n; k++) {
            n = n1 + n2;
            n2 = n1;
            n1 = n;
        }
    }

    x
}

pub fn fibo() {
    let x = 2.0;
    fn_fibo(x);
}
