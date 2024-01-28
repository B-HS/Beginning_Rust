fn main() {
    // HELLO WORLD 출력
    // println!("Hello, world!");

    // 반드시 할당되어야 사용 가능하다
    // let x: i32 = 5; // >>일단 그냥 int라 생각
    // let _y: i32; // >>일단 그냥 int라 생각
    // assert_eq!(x, 5);
    // println!("success ! ");

    // muttable, 변경가능
    // let mut x :i32 = 5; // 타입추론이 있으나 시작할때는 표시하여 연습하자
    // x+=2

    // assert_eq!(x, 7);
    // println!("success!");

    // let x: i32 = 10;

    // {
    //     let y: i32 = 5;
    //     println!("The value of x is {} and value of y is {}", x, y);
    // }
    // // let y: i32 = 12;
    // // compiling 불가, y가 scope 환경에 있고 초기화되어있지 않음
    // println!("The value of x is {} and value of y is {}", x, y);

    // define_x();

    // let x: i32 = 5;
    // {
    //     // scope 상에서만 맞음
    //     let x = 12;
    //     assert_eq!(x, 12);
    // }

    // // 해당 레벨의 스코프는 5이므로 통과
    // assert_eq!(x, 5);

    // // shadow
    // let x = 42;

    // println!("{}", x);

    // let mut x: i32 = 1;
    // x = 7; // muttable이라 수정 가능, 7로 변경
    // let mut x: i32 = x; // 7로 shadoing re-binding
    // x = x + 3; // error, immutable이기때문 >> 따라서 상단처럼 뮤터블을 다시 선언하려면 무조건 선언필요
    // println!("{}", x);

    // let y: i32 = 4;
    // let y: &str = " I can also be to text ";
    // println!("{}", y);

    // 두가지의 unused value 처리법
    // 선언시 변수명에 _ 붙이기
    // #[allow(unused_variables)]

    // 튜플 역시 immutable // mutable 영향을 받음
    // let (mut x, y) = (1, 2);
    // x += 2;

    // assert_eq!(x, 3);
    // assert_eq!(y, 2);

    // print!("success")

    // js처럼 구조분해할당 가능
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];

    assert_eq!([x, y], [3, 2]);

    print!("success")
}

// fn define_x(){
//     let x: &str = "hello";
//     println!("{}, world", x);
// }
