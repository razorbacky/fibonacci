// fibonacci Calculate . 25.01.22
// 피보나치 수열 계산
// ChatGPT 개선사항 -> 재귀를 사용하면 간단하지만 성능이 떨어질 수 있다. 그러므로 반복문을 사용한 방법이 더 효율적이다.

fn fibonacci(n: i32) -> i32 {
  // 변수를 설정한다.
  let mut a = 0;
  let mut b = 1;

  for _ in 0..n {
    let temp = b;
    b += a;
    a = temp;
  }
  a
}

fn main() {
  for i in 0..20 {
    println!("순서({i}) = {}", fibonacci(i));
  }
}