use hasura::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, hasura::Object)]
#[object(name = "customers")]
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

// let draft2 = DraftCustomer {
//     name: "John The Second".to_string(),
//     member: true,
//     email: None,
// };
// let pk = CustomerPk { c_id: 0 };
//
// let insert: Insert<Customer> = InsertBuilder::default()
//     .objects(vec![draft1.clone()])
//     // .affected_rows(true) // TODO: form is { retuning { c_id name} affected_rows }
//     .build()
//     .unwrap();
//
// let update_by_pk: UpdateByPk<Customer> = UpdateByPkBuilder::default()
//     .pk(pk)
//     .set(draft2.clone())
//     .build()
//     .unwrap();
//
// let condition = Condition {
//     op: "_eq",
//     value: "true",
// };
// let conditions = Conditions::Field(Customer::member(), vec![condition]);
//
// let customers: Query<Customer> = QueryBuilder::default()
//     // .distinct_on(Customer::name())
//     // .conditions(vec![conditions.clone()])
//     .offset(10u64)
//     .limit(10u64)
//     .build()
//     .unwrap();
//
// let url = "https://pixeltest.hasura.app/v1/graphql";
// let admin = "TAZYDFQkwpSq9YocAg47LgyjJlbB5hs1wipNjmCtRgiDSQcg9eFLW1QCOb23nS4h";
//
// let (inserted, updated) = mutation!(insert, update_by_pk)
//     .admin(Some(admin.to_owned()))
//     .send(url)
//     .await?;
//
// println!("inserted={inserted:?}\nupdated={updated:?}");
//
//
//
// println!("page={:?}", page);
// Ok(())

// let simple: Query<Quiz> = QueryBuilder::default().build().unwrap();
// println!("{}", query!(simple));

////////////////////////////////////////////////////////////////////////////////
//
// let insert = InsertBuilder::default()
//     .objects(vec![quiz.clone(), quiz.clone()])
//     .returning(Quiz::all())
//     .affected_rows(true)
//     .build()
//     .unwrap();
//
// let insert_one = InsertOneBuilder::default()
//     .object(quiz.clone())
//     .returning(Quiz::all())
//     .build()
//     .unwrap();
//
// println!("{}", insert.to_string());
// println!("{}", insert_one.to_string());
//
// let update = UpdateBuilder::default()
//     .set(quiz.clone())
//     .conditions(vec![conditions.clone()])
//     .returning(vec![Quiz::title()])
//     .build()
//     .unwrap();
//
// let update_by_pk = UpdateByPkBuilder::default()
//     .pk(QuizPk { id: 69 })
//     .set(quiz.clone())
//     .returning(vec![Quiz::title()])
//     .build()
//     .unwrap();
//
// println!("{}", update.to_string());
// println!("{}", update_by_pk.to_string());
//
// let delete = DeleteBuilder::default()
//     .conditions(vec![conditions])
//     .affected_rows(true)
//     .returning(Quiz::all())
//     .build()
//     .unwrap();
//
// let delete_by_pk: DeleteByPk<Quiz> = DeleteByPkBuilder::default()
//     .pk(QuizPk { id: 69 })
//     .returning(Quiz::all())
//     .build()
//     .unwrap();
//
// println!("{}", delete.to_string());
// println!("{}", delete_by_pk.to_string());
// }
//
