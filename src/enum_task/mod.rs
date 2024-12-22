enum ItemType{
    Book,
    Magazine,
}
pub struct Item{
    id:i32,
    title:String,
    year:i32,
    item_type:ItemType
}
pub fn display_item_info(item: &Item){
    println!("ID:{}",item.id);
    println!("Title:{}",item.title);
    println!("year:{}",item.year);
    match item.item_type {
        ItemType::Book => println!("Type: Book"),
        ItemType::Magazine => println!("Type: Magazine"),
    }

}