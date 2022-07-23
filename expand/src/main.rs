#[derive(hasura::Object, hasura::Pk)]
#[object(name = "customers", pk = "c_id")]
pub struct Customer {
    c_id: u64,
    member: bool,
}

fn main() {
}
