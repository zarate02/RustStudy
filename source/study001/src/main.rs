fn main() {

    /*
        20191217
        탭대신 스페이스 4개
        느낌표는 매크로 함수를 나타냄

        20191218
        let 은 해당 스택에서만 통용되는 변수 (=지역변수)
        변수선언시 기본적으로 불변(final) 이나 mut 를 이용해 가변으로 변경가능
    */
    let mut x = 5;
    println!("Hello, world! is {}", x);

    x = 3;
    println!("Hello, world! is {}", x);

    /*상수화와 불변은 다른개념*/
    const MAX_POINTS: u32 = 100_000;

    /*let 키워드를 사용하면 쉽게 쉐도우잉을 할수있음 */
    let y = 5;
    let y = y + 1;
    println!("Hello {}", y)
}
