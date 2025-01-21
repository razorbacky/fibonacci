// fibonacci Calculate . 25.01.21
// 피보나치 수열 계산

fn calc(n:i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        calc(n - 1) + calc(n - 2)
    }
}

fn main() {
    for i in 0..20 {
        println!("순서 {i} = {}", calc(i));
    }
}