use hasura::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DraftCustomer {
    member: bool,
    name: String,
    email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, hasura::Object, hasura::Pk)]
#[object(name = "customers", pk = "c_id", draft = "DraftCustomer")]
pub struct Customer {
    c_id: u64,
    member: bool,
    name: String,
    email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DraftItem {
    value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, hasura::Object, hasura::Pk)]
#[object(name = "customers", pk = "value", pk = "c_id")]
pub struct Item {
    s_id: u64,
    value: String,
}

fn skip_empty(x: &Data<Vec<DraftItem>>) -> bool {
    x.data.is_empty()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DraftStore {
    #[serde(skip_serializing_if = "skip_empty")]
    items: Data<Vec<DraftItem>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, hasura::Object, hasura::Pk)]
#[object(name = "customers", pk = "c_id", draft = "DraftStore")]
pub struct Store {
    s_id: u64,
    #[object(expand)]
    items: Vec<Item>,
}

#[cfg(test)]
#[test]
fn simple_query() -> Result<()> {
    let customers = QueryBuilder::default()
        .returning(Customer::all())
        .build()
        .unwrap();

    assert_eq!(
        customers.to_string(),
        "customers { c_id member name email }"
    );

    Ok(())
}

#[cfg(test)]
#[test]
fn complex_query() -> Result<()> {
    let customers = QueryBuilder::default()
        .returning(Customer::all())
        .distinct_on(Customer::name())
        .offset(10u64)
        .limit(10u64)
        .build()
        .unwrap();

    assert_eq!(
        customers.to_string(),
        "customers(distinct_on: name, limit: 10, offset: 10) { c_id member name email }"
    );

    Ok(())
}

#[cfg(test)]
#[test]
fn delete() -> Result<()> {
    let conditions = Conditions::single(Customer::name(), Eq("John"));

    let customers = DeleteBuilder::default()
        .conditions(vec![conditions])
        .returning(Customer::all())
        .build()
        .unwrap();

    assert_eq!(
        customers.to_string(),
        "delete_customers(where: { name: { _eq : \"John\" } }) { returning { c_id member name email } }"
    );

    Ok(())
}

#[cfg(test)]
#[test]
fn conditions() -> Result<()> {
    let conditions = Conditions::single(Customer::name(), Ilike("%J%"));

    let customers = DeleteBuilder::default()
        .conditions(vec![conditions])
        .returning(Customer::all())
        .build()
        .unwrap();

    assert_eq!(
        customers.to_string(),
        "delete_customers(where: { name: { _ilike : \"%J%\" } }) { returning { c_id member name email } }"
    );

    Ok(())
}

#[cfg(test)]
#[test]
fn update_by_pk() -> Result<()> {
    let customer = DraftCustomer {
        member: false,
        name: "Bert".to_string(),
        email: None,
    };
    let updated = UpdateByPkBuilder::default()
        .pk(CustomerPk { c_id: 116 })
        .set(customer)
        .returning(Customer::all())
        .build()
        .unwrap();

    assert_eq!(
        updated.to_string(),
        "update_customers_by_pk(_set: {member:false,name:\"Bert\",email:null}, pk_columns: {c_id:\"116\"}) { c_id member name email }"
    );

    Ok(())
}

#[cfg(test)]
#[test]
fn recursive_insert() -> Result<()> {
    let item0 = DraftItem {
        value: "x".to_string(),
    };
    let item1 = DraftItem {
        value: "y".to_string(),
    };

    let customer = DraftStore {
        items: Data {
            data: vec![item0, item1],
        },
    };

    let inserted = InsertOneBuilder::default()
        .object(customer)
        .returning(Store::all())
        .build()
        .unwrap();

    assert_eq!(
        inserted.to_string(),
        "insert_customers_one(object: {items:{data:[{value:\"x\"},{value:\"y\"}]}}) { s_id items { s_id value } }"
    );

    Ok(())
}

#[cfg(test)]
#[test]
fn recursive_except() -> Result<()> {
    let query = QueryBuilder::default()
        .returning(Store::except(&[Store::items(Item::all())]))
        .build()
        .unwrap();

    assert_eq!(
        query.to_string(),
        "stores { s_id }"
    );

    Ok(())
}
