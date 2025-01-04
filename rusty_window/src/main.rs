// use std::env;

fn main() {
    let a: i32 = foo();
    println!("Hello, world! {}", a);
}

fn foo () -> i32 {
    let a: i32 = 3;
    return a;
}

#[cfg(test)]
mod tests {
    use crate::foo;

    #[test]
    fn test_foo () {
        assert_eq!(foo(), 3);
    }
}

