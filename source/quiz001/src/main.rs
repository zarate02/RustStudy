/*
    아래 println 뒤에 !가 의미하는 바를 추측하시오
*/
fn main() {
    println!("99 + 100 = {}", testfunction(99,100));
}

fn testfunction(x: i32, y: i32) -> i32{
    return x+y;
}