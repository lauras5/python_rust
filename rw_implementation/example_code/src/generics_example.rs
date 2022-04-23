/*
 *  metaprogramming: a way in which the program can
 *  manipulate itself based on certain instructions
 *
 *  metaprogramming with generics
 */
struct Coordinate <T> {
    x: T,
    y: T
}

/*
 *  metaprogramming with macros
 */
macro_rules! capitalize {
    // capitalize the first letter of a string
    // altering the state of the variable
    ($a: expr) => {                                     // $a is the expression passed to macro
        let mut v: Vec<char> = $a.chars().collect();    // convert $a to vector of chars
        v[0] = v[0].to_uppercase().nth(0).unwrap();     // uppercase first letter
        $a = v.into_iter().collect();                   // convert back to string
    }
}

fn main() {
    // generics
    let one = Coordinate{x: 50, y: 50};
    let two = Coordinate{x: 500, y: 500};
    let three = Coordinate{x: 5.6, y: 5.6};

    // macros
    let mut x = String::from("test");
    capitalize!(x);
    println!("{}", x);
}

