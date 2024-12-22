// mod task_one;
mod enum_task;
mod task_func;
use enum_task::{display_item_info, Item};
use task_func::{add_3, add_5, generate_string, some_function, times};
// use crate::task_one::{task_a,task_2,task_3,task_4,task_5,task_6,task_7};
// use crate::task_func::{add_3,add_5,time};
fn main() {
    println!("Hello, rusty lang baby!");
    //     task_a();
    //     task_2();
    //     task_3();
    //     task_4();
    //    task_5();
    //    task_6();
    //    task_7();
    // let x = 3;
    // let y = 4;
    // println!(
    //     "The result of x+3 times y+5 is {}",
    //     times(add_3(x), add_5(y))
    // );
    // let s1: String = String::from("this is me, ");
    // let s2: &str = "Nouman";
    // some_function(s1.clone(), s2); // Something is wrong here
    // println!("{} {}", s1, s2);
    // let mut my_vec = vec![1, 2, 3, 4, 5];
    // let mut temp;

    // while !my_vec.is_empty() {
    //     temp = &my_vec;
    //     println!("Elements in temporary vector are: {:?}", temp);
    //     println!("Popped element: {}", my_vec.pop().unwrap()); // pop() is used to remove an element from the vec. The unwrap() is used to return the value inside the Some() variant
    // }
    // let some_book =Book{
    //     title: String::from("hell  mary")
    // };
    // print_book(some_book.title.clone()); // Fix this line
    //     println!("Book name: {}", some_book.title);
    let some_val = vec![Value::Integer(12), Value::Float(15.5)];

    for i in some_val {
        match i {
            Value::Integer(num) => println!("Integer: {} ", num),
            Value::Float(num) => println!("Float: {}", num),
        }
    }
    let my_chars = vec!['a', 'b', 'c', 'd'];
    match first_character(&my_chars) {
        Some(character) => println!("First character: {character}"),
        None => println!("Empty array"),
    }
    let user_fruit = String::from("apple");
    if let Some(fruit) = check_fruit(user_fruit) {
        println!("User's name: {fruit}");
    }
}
struct Book {
    title: String,
}
fn print_book(book_name: String) {
    println!("Book: {}", book_name);
}
// you can still extend the functionality of struct...
enum Value {
    Integer(i32),
    Float(f64),
}
fn first_character(chars: &Vec<char>) -> Option<char> {
    if chars.len() > 0 {
        Some(chars[0])
    } else {
        None
    }
}

fn check_fruit(input_fruit: String) -> Option<String> {
    let fruit_basket = vec![
        String::from("mango"),
        String::from("apple"),
        String::from("banana"),
    ];
    for fruit in fruit_basket {
        if input_fruit == fruit {
            return Some(fruit);
        }
    }
    None
}
mod m1 {
    // use m2::D;
    struct A {
        d: m2::D,
    }
    enum M {
        A,
        K,
    }
    pub mod m2 {
        // use super::M;
        pub enum D {
            B,
            C,
        }
        struct Boy {
            eye: M,
        }
    }
}
mod m3 {
    struct C {
        e: crate::m1::m2::D,
    }
}
