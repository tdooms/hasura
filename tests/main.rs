use hasura::insert::Insert;
use hasura::insert_one::InsertOne;
use hasura::query::Query;
use hasura::*;
use itertools::Either;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Hasura)]
#[hasura(table = "items")]
pub struct Article {
    #[hasura(pk)]
    name: String,

    #[hasura(pk)]
    category: String,

    price: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Hasura)]
#[hasura(table = "managers")]
pub struct Manager {
    #[hasura(pk)]
    name: String,

    weight: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Hasura)]
#[hasura(table = "stores")]
pub struct Store {
    #[hasura(pk)]
    id: Option<u64>,

    manager_id: Option<String>,

    #[hasura(relation = "Article")]
    #[serde(with = "relation")]
    items: Vec<Article>,

    #[hasura(relation = "Manager")]
    #[serde(with = "relation")]
    manager: Option<Manager>,
}

#[cfg(test)]
#[test]
fn simple_query() {
    let managers: Query<Manager> = Query::default();

    assert_eq!(managers.to_string(), "managers { name weight }");
}

#[cfg(test)]
#[test]
fn complex_query() {
    let managers: Query<Manager> = Query::default()
        .distinct_on(Manager::name())
        .offset(10u64)
        .limit(10u64);

    assert_eq!(
        managers.to_string(),
        "managers(distinct_on: name, limit: 10, offset: 10) { name weight }"
    );
}

#[cfg(test)]
#[test]
fn simple_insert() {
    let manager = Manager {
        name: "John".to_string(),
        weight: 10.0,
    };

    let insert = InsertOne::new(manager);

    assert_eq!(
        insert.to_string(),
        "insert_managers_one(object: {name:\"John\",weight:\"10\"}) { name weight }"
    );
}

#[cfg(test)]
#[test]
fn complex_insert() {
    let article0 = Article {
        name: "0".to_string(),
        category: "0".to_string(),
        price: 0,
    };
    let article1 = Article {
        name: "1".to_string(),
        category: "1".to_string(),
        price: 1,
    };

    let store0 = Store {
        id: None,
        manager_id: None,
        items: vec![article1],
        manager: None,
    };
    let store1 = Store {
        id: None,
        manager_id: None,
        items: vec![article0],
        manager: None,
    };

    let insert = Insert::new(vec![store0, store1]);

    assert_eq!(insert.to_string(), "insert_stores(objects: [{items:{data:[{name:\"1\",category:\"1\",price:\"1\"}]}}, {items:{data:[{name:\"0\",category:\"0\",price:\"0\"}]}}]) { returning { id manager_id items { name category price } manager { name weight } }")
}

// #[cfg(test)]
// #[test]
// fn delete() -> Result<()> {
//     let conditions = Conditions::single(Customer::name(), Eq("John"));
//
//     let customers = DeleteBuilder::default()
//         .conditions(vec![conditions])
//         .returning(Customer::all())
//         .build()
//         .unwrap();
//
//     assert_eq!(
//         customers.to_string(),
//         "delete_customers(where: { name: { _eq : \"John\" } }) { returning { c_id member name email } }"
//     );
//
//     Ok(())
// }
//
// #[cfg(test)]
// #[test]
// fn conditions() -> Result<()> {
//     let conditions = Conditions::single(Customer::name(), Ilike("%J%"));
//
//     let customers = DeleteBuilder::default()
//         .conditions(vec![conditions])
//         .returning(Customer::all())
//         .build()
//         .unwrap();
//
//     assert_eq!(
//         customers.to_string(),
//         "delete_customers(where: { name: { _ilike : \"%J%\" } }) { returning { c_id member name email } }"
//     );
//
//     Ok(())
// }
//
// #[cfg(test)]
// #[test]
// fn update_by_pk() -> Result<()> {
//     let customer = DraftCustomer {
//         member: false,
//         name: "Bert".to_string(),
//         email: None,
//     };
//     let updated = UpdateByPkBuilder::default()
//         .pk(CustomerPk { c_id: 116 })
//         .set(customer)
//         .returning(Customer::all())
//         .build()
//         .unwrap();
//
//     assert_eq!(
//         updated.to_string(),
//         "update_customers_by_pk(_set: {member:false,name:\"Bert\",email:null}, pk_columns: {c_id:\"116\"}) { c_id member name email }"
//     );
//
//     Ok(())
// }
//
// #[cfg(test)]
// #[test]
// fn recursive_insert() -> Result<()> {
//     let item0 = DraftItem {
//         value: "x".to_string(),
//     };
//     let item1 = DraftItem {
//         value: "y".to_string(),
//     };
//
//     let store = DraftStore {
//         items: Data {
//             data: vec![item0, item1],
//         },
//         manager: Manager::default(),
//     };
//
//     let inserted = InsertOneBuilder::default()
//         .object(store)
//         .returning(Store::all())
//         .build()
//         .unwrap();
//
//     assert_eq!(
//         inserted.to_string(),
//         "insert_stores_one(object: {items:{data:[{value:\"x\"},{value:\"y\"}]},name:\"\",size:\"0\"}) { s_id items { s_id value } manager }"
//     );
//
//     Ok(())
// }
//
// #[cfg(test)]
// #[test]
// fn recursive_except() -> Result<()> {
//     let returning = Store::except(&[Store::items(Item::all())]);
//     let query = QueryBuilder::default()
//         .returning(returning)
//         .build()
//         .unwrap();
//
//     assert_eq!(query.to_string(), "stores { s_id manager }");
//
//     Ok(())
// }
//
