#[derive(hasura::Object, hasura::Pk)]
#[object(name = "items", pk = "name")]
pub struct Item {
    name: String,
}

#[derive(hasura::Object, hasura::Pk)]
#[object(name = "customers", pk = "c_id")]
pub struct Customer {
    c_id: u64,
    member: bool,
    name: String,
    email: Option<String>,

    #[object(expand)]
    items: Vec<Item>,
}

fn main() {
    println!("Hello, world!");
}
