use hasura::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone, Hasura)]
#[hasura(table = "articles")]
pub struct Article {
    #[hasura(pk = "String")]
    name: String,

    #[hasura(pk = "String")]
    category: String,

    price: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Hasura)]
#[hasura(table = "managers")]
pub struct Manager {
    #[hasura(pk = "String")]
    name: String,

    weight: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Hasura)]
#[hasura(table = "stores")]
pub struct Store {
    #[hasura(pk = "u64")]
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
fn query_by_pk() {
    let manager: QueryByPk<Manager> = QueryByPk::new(ManagerPk{name: "Boris".into()});

    assert_eq!(manager.to_string(), "managers_by_pk(name:\"Boris\", ) { name weight }");
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

    let insert = InsertOne::new(&manager);

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

    let insert = Insert::new(vec![&store0, &store1]);

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

    let _pk = ArticlePk { name: "apple".to_string(), category: "fruits".to_string() };
    let pk = Article::pk("apple", "fruits");
    let updated = UpdateByPk::new(pk, &article);

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

//////////////////////////////////////

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hasura)]
#[hasura(table = "tags")]
pub struct Tag {
    #[hasura(pk = "u64")]
    pub quiz_id: Option<u64>,

    #[hasura(pk = "String")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Image {
    pub url: String,
    pub blurhash: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hasura, Default)]
#[hasura(table = "quizzes")]
pub struct Quiz {
    #[hasura(pk = "u64")]
    pub id: Option<u64>,

    pub public: bool,
    pub complete: bool,
    pub title: String,
    pub description: String,
    pub explanation: String,

    #[serde(default)]
    pub image: Image,

    #[hasura(relation = "Tag")]
    #[serde(with = "relation")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[cfg(test)]
#[tokio::test]
async fn simple_real() {
    dotenv::dotenv().unwrap();

    let url = std::env::var("GRAPHQL_ENDPOINT").unwrap();
    let admin = std::env::var("GRAPHQL_ADMIN_SECRET").unwrap();

    let body: Query<Quiz> = Query::new();

    query!(body).admin(admin).send(&url).await.unwrap();
}

