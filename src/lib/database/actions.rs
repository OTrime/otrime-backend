use crate::database::{DBAnswer, DBUser, Name};
use crate::http::{Password, Reveal};
use sqlx::{Pool, Postgres};

use crate::http::ids::{QuestionId, UserId};
use crate::http::models::{Answer, AnswerWithName, Question, QuestionWithName, ResponseUser};
use crate::{errors::ApiError, http::models::User};

pub async fn db_get_all_data(pool: &Pool<Postgres>) -> Result<Vec<ResponseUser>, ApiError> {
    let db_pool = pool.clone();
    let users = sqlx::query_as!(DBUser, "SELECT name, id, joined, email from users")
        .fetch_all(&db_pool)
        .await?;

    let mut users_ls = Vec::new();
    for user in users {
        let mut user: ResponseUser = user.into();
        let user_questions = db_get_questions_by_user(&db_pool, &user.id).await?;
        user.questions = user_questions;
        let user_answers = db_get_answers_by_id(&db_pool, &user.id).await?;
        user.answers = user_answers;
        let user = user;
        users_ls.push(user);
    }
    Ok(users_ls)
}

pub async fn db_create_user(pool: &Pool<Postgres>, user: &User) -> Result<ResponseUser, ApiError> {
    let db_pool = pool.clone();
    sqlx::query!(
        "INSERT INTO users (name, id, joined, password, email) VALUES ($1, $2, $3, $4, $5);",
        user.name,
        user.id.reveal(),
        user.joined.naive_utc(),
        user.password.hash()?.reveal(),
        user.email.reveal()
    )
    .execute(&db_pool)
    .await?;
    let user = sqlx::query_as!(
        DBUser,
        "SELECT name, joined, id, email from users where id = $1",
        user.id.reveal()
    )
    .fetch_one(&db_pool)
    .await?;
    let user: ResponseUser = user.into();
    Ok(user)
}

pub async fn db_create_question(
    pool: &Pool<Postgres>,
    question: &Question,
) -> Result<(), ApiError> {
    let db_pool = pool.clone();
    sqlx::query!(
        "INSERT INTO questions (title, question, id, by, asked_on) VALUES ($1, $2, $3, $4, $5)",
        question.title.clone(),
        question.question,
        question.id.reveal(),
        question.by.reveal(),
        question.asked_on.naive_utc(),
    )
    .execute(&db_pool)
    .await?;
    Ok(())
}

pub async fn db_get_questions_by_user(
    pool: &Pool<Postgres>,
    user_id: &UserId,
) -> Result<Vec<Question>, ApiError> {
    let db_pool = pool.clone();
    let record = sqlx::query_as!(
        crate::database::DBQuestion,
        "SELECT * FROM questions WHERE by = $1",
        user_id.reveal()
    )
    .fetch_all(&db_pool)
    .await?;

    let mut questions: Vec<Question> = Vec::new();
    for question in record {
        let answers = sqlx::query_as!(
            DBAnswer,
            "SELECT * FROM answers where for_question = $1",
            question.id
        )
        .fetch_all(&db_pool)
        .await?;

        let mut question: Question = question.into();
        for answer in answers {
            let answer: Answer = answer.into();
            question.answer.push(answer);
        }

        questions.push(question);
    }

    Ok(questions)
}

pub async fn db_get_user_data(
    pool: &Pool<Postgres>,
    user: &UserId,
) -> Result<ResponseUser, ApiError> {
    let db_pool = pool.clone();
    let user = sqlx::query_as!(
        DBUser,
        "SELECT name, joined, id, email from users where id = $1;",
        user.reveal()
    )
    .fetch_one(&db_pool)
    .await?;

    let mut user: ResponseUser = user.into();
    for mut i in db_get_questions_by_user(&db_pool, &user.id).await? {
        let answers = db_get_answers(&db_pool, &i.id).await?;
        i.answer = answers;
        user.questions.push(i);
    }

    for a in db_get_answers_by_id(&db_pool, &user.id).await? {
        user.answers.push(a)
    }

    Ok(user)
}

pub async fn db_get_user_data_using_email(
    pool: &Pool<Postgres>,
    user: String,
) -> Result<ResponseUser, ApiError> {
    let db_pool = pool.clone();
    let user = sqlx::query_as!(
        DBUser,
        "SELECT name, joined, id, email from users where email = $1;",
        user
    )
    .fetch_one(&db_pool)
    .await?;

    let mut user: ResponseUser = user.into();
    for mut i in db_get_questions_by_user(&db_pool, &user.id).await? {
        let answers = db_get_answers(&db_pool, &i.id).await?;
        i.answer = answers;
        user.questions.push(i);
    }

    Ok(user)
}

pub async fn db_get_answers(
    pool: &Pool<Postgres>,
    question_id: &QuestionId,
) -> Result<Vec<Answer>, ApiError> {
    let db_clone = pool.clone();
    let answer = sqlx::query_as!(
        DBAnswer,
        "SELECT * FROM answers where for_question = $1",
        question_id.reveal()
    )
    .fetch_all(&db_clone)
    .await?;

    let mut answers: Vec<Answer> = Vec::new();

    for i in answer {
        let answer: Answer = i.into();
        answers.push(answer);
    }
    Ok(answers)
}

pub async fn db_create_answer(
    pool: &Pool<Postgres>,
    answer: &Answer,
) -> Result<ResponseUser, ApiError> {
    let db_pool = pool.clone();
    sqlx::query!(
        "
    INSERT INTO answers(answer, id, for_question, by, answered_on) VALUES ($1, $2, $3, $4, $5);
    ",
        answer.answer,
        answer.id.reveal(),
        answer.for_question.reveal(),
        answer.by.reveal(),
        answer.answered_on.naive_utc()
    )
    .execute(&db_pool)
    .await?;
    let user = db_get_user_data(&db_pool, &answer.by).await?;
    Ok(user)
}

pub async fn db_get_user_password(
    pool: &Pool<Postgres>,
    email: &str,
) -> Result<Password, ApiError> {
    let db_pool = pool.clone();
    let password_hash = sqlx::query!("SELECT password from users where email = $1", email)
        .fetch_one(&db_pool)
        .await?;
    let password = Password::new(password_hash.password);
    Ok(password)
}

pub async fn db_get_answers_by_id(
    pool: &Pool<Postgres>,
    user: &UserId,
) -> Result<Vec<Answer>, ApiError> {
    let db_pool = pool.clone();
    let answers = sqlx::query_as!(
        DBAnswer,
        "SELECT * from answers where by = $1",
        user.reveal()
    )
    .fetch_all(&db_pool)
    .await?;

    let mut ans = vec![];
    for i in answers {
        let answer: Answer = i.into();
        ans.push(answer);
    }
    Ok(ans)
}

pub async fn db_get_name(pool: &Pool<Postgres>, user: &UserId) -> Result<String, ApiError> {
    let db_pool = pool.clone();
    let name = sqlx::query!("SELECT name FROM users where id = $1", user.reveal())
        .fetch_one(&db_pool)
        .await?;
    Ok(name.name)
}

pub async fn db_get_answers_list(
    pool: &Pool<Postgres>,
    question_id: &QuestionId,
) -> Result<Vec<AnswerWithName>, ApiError> {
    let db_pool = pool.clone();
    let answers = sqlx::query_as!(
        DBAnswer,
        "SELECT * FROM answers where for_question = $1",
        question_id.reveal()
    )
    .fetch_all(&db_pool)
    .await?;

    let mut answer_list = vec![];

    for answer in answers {
        let name = sqlx::query!("SELECT name from users where id = $1", answer.by)
            .fetch_one(&db_pool)
            .await?;
        let answer: Answer = answer.into();
        let mut answer: AnswerWithName = answer.into();
        answer.name = name.name;
        answer_list.push(answer);
    }
    Ok(answer_list)
}

pub async fn db_get_question_list(
    pool: &Pool<Postgres>,
) -> Result<Vec<QuestionWithName>, ApiError> {
    let db_pool = pool.clone();
    let questions = sqlx::query_as!(crate::database::DBQuestion, "SELECT * FROM questions")
        .fetch_all(&db_pool)
        .await?;

    let mut questlist = vec![];
    for i in questions {
        let question: Question = i.into();
        let mut question: QuestionWithName = question.into();
        let answer_list = db_get_answers_list(&db_pool, &question.id).await?;
        question.answer = answer_list;
        let name = sqlx::query_as!(
            Name,
            "SELECT name from users where id = $1",
            question.by.reveal()
        )
        .fetch_one(&db_pool)
        .await?;
        question.name = name.name;
        questlist.push(question);
    }
    Ok(questlist)
}

pub async fn db_get_question_by_id(
    pool: &Pool<Postgres>,
    question_id: &QuestionId,
) -> Result<QuestionWithName, ApiError> {
    let db_pool = pool.clone();
    let question = sqlx::query_as!(
        crate::database::DBQuestion,
        "SELECT * from questions where id = $1",
        question_id.reveal()
    )
    .fetch_one(&db_pool)
    .await?;

    let question: Question = question.into();
    let mut question: QuestionWithName = question.into();

    let name = sqlx::query_as!(
        Name,
        "SELECT name from users where id = $1",
        question.by.reveal()
    )
    .fetch_one(&db_pool)
    .await?;

    question.name = name.name;

    let answers = sqlx::query_as!(
        DBAnswer,
        "SELECT * from answers where for_question = $1",
        question.id.reveal()
    )
    .fetch_all(&db_pool)
    .await?;

    for answer in answers {
        let answer: Answer = answer.into();
        let mut answer: AnswerWithName = answer.into();
        let name = db_get_name(&pool, &answer.by).await?;
        answer.name = name;
        question.answer.push(answer);
    }

    Ok(question)
}

pub async fn validate_user_id(pool: &Pool<Postgres>, user: &UserId) -> Result<bool, ApiError> {
    let db_pool = pool.clone();

    let _id = sqlx::query!("SELECT id from users where id = $1", user.reveal())
        .fetch_one(&db_pool)
        .await?;

    Ok(true)
}
