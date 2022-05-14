use chrono::{DateTime, Utc};

use super::ids::*;
use super::{Email, Password};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestAnswer {
    pub by: String,
    pub for_question: String,
    pub answer: String,
}




#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestUserId {
    pub user_id: String

}

impl From<RequestUserId> for UserId {
    fn from(user: RequestUserId) -> Self {
       user.user_id.into()
    
    }
}
impl From<RequestAnswer> for Answer {
    fn from(answer: RequestAnswer) -> Self {
        Self {
            answer: answer.answer,
            by: answer.by.into(),
            for_question: answer.for_question.into(),
            id: AnswerId::new(),
            answered_on: Utc::now(),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestUserQuestion {
    pub question_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub email: Email,
    pub joined: DateTime<Utc>,
    pub id: UserId,
    pub questions: Vec<Question>,
    pub name: String,
    pub password: Password,
    pub answers: Vec<Answer>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestQuestion {
    pub question: String,
    pub by: String,
    pub title: String,
}

impl From<RequestQuestion> for Question {
    fn from(q: RequestQuestion) -> Self {
        Self {
            title: q.title,
            question: q.question,
            by: q.by.into(),
            answer: Vec::new(),
            asked_on: Utc::now(),
            id: QuestionId::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseUser {
    pub email: Email,
    pub joined: DateTime<Utc>,
    pub id: UserId,
    pub questions: Vec<Question>,
    pub name: String,
    pub answers: Vec<Answer>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct Question {
    pub title: String,
    pub question: String,
    pub answer: Vec<Answer>,
    pub asked_on: DateTime<Utc>,
    pub by: UserId,
    pub id: QuestionId,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuestionWithName {
    pub question: String,
    pub answer: Vec<AnswerWithName>,
    pub asked_on: DateTime<Utc>,
    pub by: UserId,
    pub id: QuestionId,
    pub name: String,
    pub title: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Answer {
    pub answer: String,
    pub for_question: QuestionId,
    pub by: UserId,
    pub id: AnswerId,
    pub answered_on: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnswerWithName {
    pub answer: String,
    pub for_question: QuestionId,
    pub by: UserId,
    pub id: AnswerId,
    pub answered_on: DateTime<Utc>,
    pub name: String,
}

impl From<Question> for QuestionWithName {
    fn from(q: Question) -> Self {
        let mut answers: Vec<AnswerWithName> = vec![];
        for answer in q.answer {
            answers.push(answer.into())
        }
        Self {
            title: q.title,
            question: q.question,
            answer: answers,
            asked_on: q.asked_on,
            by: q.by,
            id: q.id,
            name: String::new(),
        }
    }
}

impl From<Answer> for AnswerWithName {
    fn from(a: Answer) -> Self {
        Self {
            answer: a.answer,
            for_question: a.for_question,
            by: a.by,
            id: a.id,
            answered_on: a.answered_on,
            name: String::new(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignInUser {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestAuth {
    pub session_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl From<RegisterUser> for User {
    fn from(user: RegisterUser) -> Self {
        Self {
            name: user.name,
            email: user.email.into(),
            id: UserId::new(),
            joined: Utc::now(),
            questions: Vec::new(),
            password: user.password.into(),
            answers: Vec::new(),
        }
    }
}
impl From<User> for ResponseUser {
    fn from(user: User) -> Self {
        Self {
            name: user.name,
            email: user.email,
            id: user.id,
            joined: user.joined,
            questions: user.questions,
            answers: user.answers,
        }
    }
}

impl From<String> for Email {
    fn from(em: String) -> Self {
        Self(em)
    }
}

impl From<String> for Password {
    fn from(em: String) -> Self {
        Self(em)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientEmail {
    pub name: String,
    pub joined: String,
    pub email: String,
}
