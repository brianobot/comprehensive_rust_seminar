struct Foo {
    x: i32,
    y: u32,
}

enum Direction {
    Up(i32),
    Down(i32),
    Right(i32),
    Left(i32),
}

fn main() {
    let foo = Foo { x: 1, y: 3 };
    let Foo { x: a, y: b } = foo;

    println!("a={a}, b={b}");

    // the same concept can be applied in match arms to destructure structs and capture the fields
    match foo {
        Foo { x, y } => {
            println!("X={x} Y={y}")
        }
    }

    // the same logic can be applied to enums
    let double_right = Direction::Right(2);
    // let Direction::Right(num_times) = double_right; // this would not work with enums
    // currently i can't explain why, but if you wanted a similar thing you would have to use a match expression to get it done
    // Update: this does not work, because the match is not refutable since there are different variants of the Direction enum

    match double_right {
        Direction::Up(n) => println!("Moving up {n} times"),
        Direction::Down(n) => println!("Moving down {n} times"),
        Direction::Right(n) => println!("Moving right {n} times"),
        Direction::Left(n) => println!("Moving left {n} times"),
    }
}
