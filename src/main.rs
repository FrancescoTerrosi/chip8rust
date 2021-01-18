use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let mut j:i32= 0;
    for i in 0..5 as i32 {
        println!("{}", i);
        while j <= i {
            let mut to_print= if j % 2 == 0 {"Viva diahane! ".to_owned() + &j.to_string()} else {"Sono dispari diahane!".to_string()};
            j += 1;
            println!("{}", to_print);
        }
        j = 0;
    }

    let result = int_to_float(10);
    println!("{}", type_of(result));
    println!("{}", result);

    println!("{}", factorial(11));

    let x: f64 = 0.0;

    let result = compute_cos(x);

    println!("cosine of {} = {}", x, result);
    println!("cosine of {} = {}", result, result.cos());

    let a = [1,2,3,4,5];
    println!("Accumulate: {}", accumulate(&a));
    let x1 = a.get(0);
    let x2 = a.get(7);
    println!("{:?}", x1);
    println!("{:?}", x2);

    let y1 = x1.unwrap_or(&-1);
    let y2 = x1.unwrap_or(&-1);

    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    println!("{:?}", &v);

    for o in v.iter() {
        println!("{}", o);
    }


    println!("{}", accumulate(&a));
    println!("{}", pro_accumulate(&a));

    let lm = (1..11);

    for l in lm {
        println!("{:?}", l);
    }

    v.de

}


fn int_to_float(x: i32) -> f64 {
    x as f64
}

fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn compute_cos(x: f64) -> f64 {
    f64::cos(x)
}

fn compute_circumference(r: f64) -> f64 {
    let result = r.powi(2)* std::f64::consts::PI;
    let check = r * r * std::f64::consts::PI;
    assert_eq!(check, result);
    result
}

fn accumulate(a: &[i32]) -> i32 {
    let mut result = 0;
    for i in 0..a.len() {
        result += a[i];
    }
    return result;
}

fn pro_accumulate(a: &[i32]) -> i32 {
    a.iter().sum()
}