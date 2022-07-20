use hasura::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Object, Pk)]
#[object(name = "rounds", pk = "quiz_id", pk = "index")]
pub struct Round {
    pub quiz_id: u32,
    pub index: u32,

    pub answer: String,
    pub speed: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Object, Pk)]
#[object(name = "users", pk = "id")]
pub struct Creator {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Object, Pk)]
#[object(name = "quizzes", pk = "id")]
pub struct Quiz {
    pub id: u32,
    pub public: bool,
    pub complete: bool,

    pub title: String,
    pub description: String,
    pub explanation: String,

    #[object(expand)]
    pub creator: Creator,
}

#[cfg(test)]
#[tokio::test]
pub async fn full_quiz() {
    let condition = Eq("102");
    let conditions = Conditions::single(Round::quiz_id(), condition);

    let quiz = QueryByPkBuilder::default()
        .pk(QuizPk { id: 102 })
        .returning(Quiz::all())
        .build()
        .unwrap();

    let rounds: Query<Round> = QueryBuilder::default()
        .conditions(vec![conditions])
        .returning(Round::all())
        .build()
        .unwrap();

    println!("{quiz}");
    // let quiz = query!(quiz).admin(admin).send(endpoint).await.unwrap();
    // println!("{quiz:?}, {rounds:?}");
}
