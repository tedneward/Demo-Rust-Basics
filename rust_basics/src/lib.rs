#[allow(dead_code)]

// {{## BEGIN basics ##}}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
// {{## END basics ##}}

// {{## BEGIN comments ##}}
// This is a standard comment

/// This is a documentation comment.
/// It supports Markdown syntax.
/// 
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = rust_basics::add(5, arg);
///
/// assert_eq!(10, answer);
/// ```
///
/// NOTE: rust_doc will sanity-check the code inside
/// the Examples part of these comments!
// {{## END comments ##}}


// {{## BEGIN variables ##}}
pub fn immutable_x() -> i32 {
    let x = 5;
    //x = 6; // ERROR!
    println!("The value of x is: {x}");
    return x;
}
pub fn redeclaring_x() -> i32 {
    let x = 5;
    println!("The value of x is: {x}");
    let x = x + 1;
    println!("The value of x is: {x}");
    return x;
}
pub fn mutable_x() -> i32 {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    return x;
}
// {{## END variables ##}}

// {{## BEGIN scalars ##}}
pub fn math_on_scalars() {
    // addition
    let _sum = 5 + 10; // unused, so prefix with "_"

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder/modulo
    let _remainder = 43 % 5;
}
// {{## END scalars ##}}

// {{## BEGIN loop ##}}
fn loop_example() -> i32 {
    let mut counter = 0;

    let result = loop {
        println!("Looping on {counter}...");
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    return result;
}
// {{## END loop ##}}

// {{## BEGIN while ##}}
fn while_countdown() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    };

    println!("LIFTOFF!!!");
}
// {{## END while ##}}

// {{## BEGIN for ##}}
fn for_examples() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // Use a Range object, reversed
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
// {{## END for ##}}

// {{## BEGIN tuple ##}}
fn tuple_fun() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring

    println!("The value of y is: {y}");
}
// {{## END tuple ##}}

// {{## BEGIN function ##}}
fn function_call() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    //x + 1   // NOTE: semicolon would make this a statement
            // and therefore not an implicit return value
            // and therefore an error!
    return x + 1      // acceptable
    //return x + 1;     // acceptable
}// {{## END function ##}}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn variables_return_correctly() {
        let result = immutable_x();
        assert_eq!(result, 5);
        let result = redeclaring_x();
        assert_eq!(result, 6);
        let result = mutable_x();
        assert_eq!(result, 6);
    }
}
