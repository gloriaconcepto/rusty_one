pub  fn task_a(){
    let my_age  = 40;
    println!("My age is: {}", my_age);  // do not change this line
 }

 pub  fn task_2(){
    let mut x1 =40;
    let mut  x2 =x1;
    x2=x1-2;
    println!("x1 is: {} and x2 is: {}", x1,x2);
 }

 pub  fn task_3(){
    let mut x1 = 40;
    let x2;
    x1 = x1 * 3;
    x2 = x1 - 2;
    println!("x1 is: {}, x2 is: {}", x1, x2); // yes it will complie...
 }

 pub  fn task_4(){
    let a = "three";  // don't change this line
    let a = 10; // don't change the name of this variable this is to apply the concept of shallowing...
    println!("a is: {}", a); 
 }

 pub  fn task_5(){
    let x: u8; // Don't change this line!
    x = 1;
    println!("x is: {}", x);
 }
 pub fn task_6(){
    let pi: f64;
    pi = 3.14159; // This value represents pi 
    println!("pi is: {}", pi);
 }
 pub fn task_7(){
    type Book =(String,String,i32); // Add your code here to this line

    let book1: Book = (
        String::from("Rust Programming Langauge"),
        String::from("RUST Community"),
        2010,
    );
    println!(
        "Book name: {}, Author: {}, Year {}",
        book1.0, book1.1, book1.2
    );

    let book2: Book = (
        String::from("Rust by Example"),
        String::from("Steve Klabnik and Carol Nichols"),
        2015,
    );
    println!(
        "Book name: {}, Authors: {}, Year {}",
        book2.0, book2.1, book2.2
    );
 }