use crate::List::{Cons, Nil};
use std::ops::Deref;

fn r#box() {
    let b = Box::new(5);
    println!("b = {}", b);
}

// Rust can't figure out how much space to allocate for recursively defined types, so the compiler gives
// the error
enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_works() {
        let b = Box::new(5);
        println!("Box: {}", b);
    }

    #[test]
    fn list_works() {
        // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)));

        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }

    #[test]
    fn reference_test() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn reference_test2() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y); //*(y.deref())
    }

    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    #[test]
    fn test_hello() {
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
    }

    #[test]
    fn test_dropping() {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created.");
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }
}

// boxes have a know size
// construct function (cons function)
