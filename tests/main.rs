use hasura::delete::Delete;
use hasura::insert::Insert;
use hasura::insert_one::InsertOne;
use hasura::query::Query;
use hasura::*;
use serde::{Deserialize, Serialize};
use hasura::update_by_pk::UpdateByPk;

#[derive(Debug, Serialize, Deserialize, Clone, Hasura)]
#[hasura(table = "articles")]
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
    articles: Vec<Article>,

    #[hasura(relation = "Manager")]
    #[serde(with = "relation")]
    manager: Option<Manager>,
}

#[cfg(test)]
#[test]
fn simple_query() {
    let managers: Query<Manager> = Query::new();

    assert_eq!(managers.to_string(), "managers { name weight }");
}

#[cfg(test)]
#[test]
fn complex_query() {
    let managers: Query<Manager> = Query::new()
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
        articles: vec![article1],
        manager: None,
    };
    let store1 = Store {
        id: None,
        manager_id: None,
        articles: vec![article0],
        manager: None,
    };

    let insert = Insert::new(vec![store0, store1]);

    assert_eq!(insert.to_string(), "insert_stores(objects: [{articles:{data:[{name:\"1\",category:\"1\",price:\"1\"}]}}, {articles:{data:[{name:\"0\",category:\"0\",price:\"0\"}]}}]) { returning { id manager_id articles { name category price } manager { name weight } } }")
}

#[cfg(test)]
#[test]
fn simple_delete() {
    let conditions = Conditions::single(Article::name(), Eq("apple"));
    let articles = Delete::new().conditions(conditions);

    assert_eq!(
        articles.to_string(),
        "delete_articles(where: { name: { _eq : \"apple\" } }) { returning { name category price } }"
    )
}

#[cfg(test)]
#[test]
fn conditions() {
    let conditions = Conditions::single(Manager::name(), Ilike("%J%"));
    let managers = Delete::new().conditions(conditions);

    assert_eq!(
        managers.to_string(),
        "delete_managers(where: { name: { _ilike : \"%J%\" } }) { returning { name weight } }"
    )
}

#[cfg(test)]
#[test]
fn update_by_pk() {
    let article = Article {
        name: "apple".to_string(),
        category: "fruits".to_string(),
        price: 7,
    };

    let pk = ArticlePk { name: "apple".to_string(), category: "fruits".to_string() };
    let updated = UpdateByPk::new(pk, article);

    assert_eq!(
        updated.to_string(),
        "update_articles_by_pk(pk_columns: {name:\"apple\",category:\"fruits\"}, _set: {name:\"apple\",category:\"fruits\",price:\"7\"}) { name category price }"
    );
}

#[cfg(test)]
#[test]
fn recursive_except() {
    let returning = Store::except(&[Store::articles(Article::all())]);
    let query = Query::new().returning(returning);

    assert_eq!(query.to_string(), "stores { id manager_id manager { name weight } }");
}

