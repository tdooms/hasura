use api::*;

// #[derive(Clone, derive::Object)]
// #[object(name = "quizzes", pk = "id")]
// struct Quiz {
//     id: i32,
//     title: String,
//     public: bool,
// }

#[derive(Clone, derive::Object, derive::Pk)]
#[name("quizzes")]
#[pk("index", "quiz_id")]
struct Round {
    index: u64,
    quiz_id: u64,
    question: String,
    image: Option<String>,
}

#[test]
fn main() {
    let round = Round {
        index: 420,
        quiz_id: 69,
        question: "What is your name?".to_string(),
        image: None,
    };

    let insert = InsertBuilder::default()
        .objects(vec![round.clone()])
        .returning(Round::all())
        .affected_rows(true)
        .build()
        .unwrap();

    let update_by_pk = UpdateByPkBuilder::default()
        .pk(RoundPk {
            index: 420,
            quiz_id: 69,
        })
        .set(round.clone())
        .returning(Round::all())
        .build()
        .unwrap();

    println!("{}", insert.to_string());
    println!("{}", update_by_pk.to_string());

    // let quiz = Quiz {
    //     id: 1,
    //     title: "Rust".to_string(),
    //     public: true,
    // };
    //
    // let condition = Condition {
    //     op: "_eq",
    //     value: "true",
    // };
    // let conditions = Conditions::Field(Quiz::title(), vec![condition]);
    //
    // let query = QueryBuilder::default()
    //     .distinct_on(Quiz::title())
    //     .conditions(vec![conditions.clone()])
    //     .offset(10u64)
    //     .limit(10u64)
    //     .returning(Quiz::all())
    //     .build()
    //     .unwrap();
    //
    // println!("{}", query.to_string());
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
}
