#[test]
fn test() {
    assert_eq!(2 + 3, 5);
}

#[test]
fn test1() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}

#[test]
fn test2() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}

#[test]
fn test3() {
    let x: i32 = 10;
    let y: i32;
    {
        y = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}