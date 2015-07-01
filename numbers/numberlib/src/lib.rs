use std::char;
use std::str;

fn calculated_pi_2(places: usize) -> f64 {
    let mut generator: Vec<isize> = Vec::with_capacity((10*places/3)+1);

    loop {
        
    }
}

fn calculated_pi(places: usize) -> f64 {
    let mut q = 1i64;
    let mut r = 0i64;
    let mut t = 1i64;
    let mut k = 1i64;
    let mut n = 3i64;
    let mut l = 3i64;

    let mut digits: Vec<char> = Vec::new();
    loop {
        if n*t > 4i64*q+r-t {
            // Capture n at this stage.
            println!("digit: {}", n);
            digits.push(char::from_digit(n as u32, 10).unwrap());
            if digits.len() >= places {
                break;
            }
            println!("r: {}, n: {}, t: {}", r,n,t);
            let nr = 10i64 * (r - n * t);
            n = ((10i64 * (3i64 * q + r)) / t) - 10i64 * n;
            q *= 10i64;
            r = nr;
        } else {
            let nr = (2i64 * q + r) * l;
            let nn = (q * (7i64 * k + 2i64) + r * l) / (t * l);
            q *= k;
            t *= l;
            l += 2i64;
            k += 1i64;
            n = nn;
            r = nr;
        }
    }
    digits.insert(1, '.');
    digits.iter().cloned().collect::<String>().parse::<f64>().unwrap()
}

fn calculated_e(digits: usize) -> f64 {
    3f64
}



#[test]
fn pi_to_nth() {
    assert_eq!(3f64, calculated_pi(1));
    //assert_eq!(3.1f64, calculated_pi(2));
   // assert_eq!(3.14f64, calculated_pi(3));
    assert_eq!(3.1415926535897932f64, calculated_pi(17));
}

#[test]
fn e_to_nth() {
    assert_eq!(3f64, calculated_e(1));
    assert_eq!(2.7f64, calculated_e(2));
    assert_eq!(2.72f64, calculated_e(3));
}
