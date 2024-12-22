
pub fn add_3(x:i64) -> i64{
    3+x
}

pub fn add_5(x:i64) -> i64{
    5+x
}

pub fn times(x:i64,y:i64)-> i64{
    x*y
}

pub fn some_function(a1:String,a2:&str){
    println!("{} {}", a1, a2)
}
pub fn generate_string() -> String{
    let some_string = String::from("I wiil generate a volley ball");
    some_string
}

struct Fruit{
    apples: i32,
    bananas: i32
}
fn increase_fruit(fruit:Fruit)-> Fruit{
  Fruit {
        apples: fruit.apples * 2,
        bananas: fruit.bananas * 3,
    }

}