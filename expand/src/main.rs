use hasura::Hasura;

#[derive(Hasura)]
#[hasura(table = "managers")]
pub struct Manager {
    #[hasura(pk)]
    name: String,

    weight: f64,
}

#[derive(Hasura)]
#[hasura(table = "stores")]
pub struct Store {
    #[hasura(pk)]
    id: Option<u64>,

    #[hasura(relation = "Manager")]
    manager: Option<Manager>,
}

fn main() {
}
