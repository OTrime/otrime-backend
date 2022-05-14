pub mod actions;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;

use crate::http::{
    models::{Answer, Question, ResponseUser},
};

#[derive(Clone, Debug)]
pub struct Database {
    pub pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(uri: &str) -> Self {
        Self {
            pool: PgPoolOptions::new().connect(uri).await.unwrap(),
        }
    }
}

pub struct DBUser {
    pub email: String,
    pub joined: NaiveDateTime,
    pub id: Uuid,
    pub name: String,
}

impl From<DBUser> for ResponseUser {
    fn from(user: DBUser) -> Self {
        let d: DateTime<Utc> = Utc.from_utc_datetime(&user.joined);
        Self {
            email: user.email.into(),
            joined: d,
            id: user.id.into(),
            questions: Vec::new(),
            name: user.name,
            answers: Vec::new(),
        }
    }
}

pub struct DBQuestion {
    pub title: String,
    pub asked_on: NaiveDateTime,
    pub by: Uuid,
    pub id: Uuid,
    pub question: String,
}

impl From<DBQuestion> for Question {
    fn from(a: DBQuestion) -> Self {
        let d: DateTime<Utc> = Utc.from_utc_datetime(&a.asked_on);
        Self {
            title: a.title,
            asked_on: d,
            question: a.question,
            answer: vec![],
            id: a.id.into(),
            by: a.by.into(),
        }
    }
}

pub struct DBAnswer {
    pub answer: String,
    pub for_question: Uuid,
    pub by: Uuid,
    pub id: Uuid,
    pub answered_on: NaiveDateTime,
}

impl From<DBAnswer> for Answer {
    fn from(a: DBAnswer) -> Self {
        let d: DateTime<Utc> = Utc.from_utc_datetime(&a.answered_on);
        Self {
            answer: a.answer,
            for_question: a.for_question.into(),
            by: a.by.into(),
            id: a.id.into(),
            answered_on: d,
        }
    }
}

pub struct Name {
    pub name: String,
}
