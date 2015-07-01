fn fib(upper_bound: u32) -> u32 {
    let mut a = 1;
    let mut b = 1;
    while a < upper_bound {
        let temp = a.clone();
        a += b;
        b = temp;
    }
    return { if a > upper_bound { b } else { a } };
}

#[test]
fn test_fib() {
    assert_eq!(1, fib(1));
    assert_eq!(2, fib(2));
    assert_eq!(8, fib(10));
    assert_eq!(987, fib(1000));
}
