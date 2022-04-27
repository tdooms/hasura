use api::*;

#[derive(Clone, hasura::Object, hasura::Encode)]
#[object(name = "creators")]
struct Creator {
    name: String,
    image: Option<String>,
}

#[derive(Clone, hasura::Object, hasura::Pk, derive::Encode)]
#[object(name = "quizzes", pk = "id", draft = "DraftRound")]
struct Quiz {
    id: i32,

    title: String,
    public: bool,

    #[object(expand)]
    creator: Creator,
}

#[derive(hasura::Encode, Clone)]
struct DraftRound {
    title: String,
    public: bool,
    creator: Creator,
}

#[derive(Clone, hasura::Object, hasura::Pk, hasura::Encode)]
#[object(name = "rounds", pk = "index", pk = "quiz_id")]
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

    let draft = DraftRound {
        title: "MyGMP".to_string(),
        public: false,
        creator: Creator {
            name: "Ikke".to_string(),
            image: None,
        },
    };

    let round_pk = RoundPk {
        index: 420,
        quiz_id: 69,
    };

    let insert = InsertBuilder::default()
        .objects(vec![draft.clone()])
        .returning(Quiz::all())
        .affected_rows(true)
        .build()
        .unwrap();

    let update_by_pk = UpdateByPkBuilder::default()
        .pk(round_pk)
        .set(round.clone())
        .returning(Round::all())
        .build()
        .unwrap();

    println!("{}", mutation!(update_by_pk, insert));

    let condition = Condition {
        op: "_eq",
        value: "true",
    };
    let conditions = Conditions::Field(Quiz::title(), vec![condition]);

    let quizzes: Query<Quiz> = QueryBuilder::default()
        .distinct_on(Quiz::title())
        .conditions(vec![conditions.clone()])
        .offset(10u64)
        .limit(10u64)
        .returning(Quiz::all())
        .build()
        .unwrap();

    println!("{}", query!(quizzes));

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
