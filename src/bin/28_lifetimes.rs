#![allow(dead_code, unused_variables, unused_mut, unused_assignments)]

fn main() {
    // in the simplest cases, borrow last for the duration of the function that borrows it
    //
    let mut val = 23;

    borrows(&val);

    val += 5; // this is allowed, because at this point, the reference to val has been freed 
    // but we can also return references from functions
    let mut x = 43;
    let out = echo(&x);

    // x += 5; // we can not do this, because a reference to the value is still held by out in the current scope below

    dbg!(out);

    // in the case of the mutiple_borrow function
    // the issue is, how can the compiler tell which reference is returned
    let mut a = 5;
    let mut b = 6;

    // let r = multiple_borrow_error(&a, &b);
    // checking borrow, the compiler does not look at the body of the function that borrows to reason about the borrow
    // instead it looks only at the signature of the function for borrow analysis
    //

    // a += 1;
    // b += 1;

    // dbg!(r);
    //

    // to solve this we can borrow both references for the return value
    let r = multiple_borrow_fix(&a, &b);

    // in this case we can not mutate any of the references until both references are free
    let _ = drop(r); // at this point the reference is freed and both values can be mutated again
    // calls to std::mem::drop with a reference instead of an owned value does nothing
    

    a += 1;
    b += 3;

    dbg!(a, b);

    let mut message = String::from("This is my beloved Son in whom I am well pleased");
    let point_1 = &message[19..22];
    let point_2 = &message[41..];

    println!("points: {:?}, {:?}", point_1, point_2);
    let hight_1 = Highlight {
        slice: point_1,
        color: HighlightColor::GREEN,
    };
    let hight_2 = Highlight {
        slice: point_2,
        color: HighlightColor::GREY,
    }; // this works because at this point the point_1 and point_2 references are still alive

    println!("{:?}", point_1); // this also works because we do not reference the highlight strcuture below here
    // but if we did, the line above would invalidate the reference and the slice would become unreachable
    // the slice field continues to borrow the reference from the initial source message
    // we can not mutate the source and still use the reference, this would lead to a compiler error
}

fn borrows(x: &i32) {
    dbg!(x);
}

fn echo(x: &i32) -> &i32 {
    x
}

// fn multiple_borrow_error(a: &i32, b: &i32) -> &i32 {
//     if a > b { a } else { b };
// }

fn multiple_borrow_fix<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if a > b { b } else { a }
}

#[allow(dead_code)]
#[derive(Debug)]
enum HighlightColor {
    GREEN,
    YELLOW,
    GREY,
    RED,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Highlight<'document> {
    slice: &'document str, // this is basically saying that Highlight struct would continue to lock the document source until
    // so if you want to mutate the the document source, highlight must be dropped to release the lock
    color: HighlightColor,
}
