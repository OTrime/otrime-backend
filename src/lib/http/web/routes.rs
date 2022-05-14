use crate::{
    database::actions::{
        db_create_answer, db_create_question, db_get_question_by_id, db_get_question_list,
        db_get_user_data, db_get_user_data_using_email, db_get_user_password, db_get_all_data,
    },
    errors::ApiError,
    http::{
        ids::{QuestionId, UserId},
        models::{
            Answer, Question, QuestionWithName, RegisterUser, RequestAnswer, RequestQuestion,
            RequestUserId, RequestUserQuestion, ResponseUser, SignInUser,
        },
        Reveal,
    },
    State,
};

use std::sync::Arc;
use tower_cookies::{Cookie, Cookies};
use axum::http::header::{HeaderMap, COOKIE};

use axum::{Extension, Json};
pub async fn create_user(
    user: Json<RegisterUser>,
    Extension(data): Extension<Arc<State>>,
) -> Result<Json<ResponseUser>, ApiError> {
    let user = user.0.into();
    let pool = data.database.pool.clone();
    let user = crate::database::actions::db_create_user(&pool, &user).await?;
    Ok(Json(user))
}


pub async fn get_user_data_using_id(
    user_id: Json<RequestUserId>,
    Extension(data): Extension<Arc<State>>,
) -> Result<Json<ResponseUser>, ApiError> {
    let pool = data.database.pool.clone();
    let user_id: UserId = user_id.user_id.clone().into();
    let user_data = db_get_user_data(&pool, &user_id).await?;
    Ok(Json(user_data))
}

pub async fn create_question(
    question: Json<RequestQuestion>,
    Extension(data): Extension<Arc<State>>,
) -> Result<Json<ResponseUser>, ApiError> {
    let pool = data.database.pool.clone();
    let question: Question = question.0.into();
    db_create_question(&pool, &question).await?;

    let user = db_get_user_data(&pool, &question.by).await?;
    Ok(Json(user))
}

pub async fn home(Extension(data): Extension<Arc<State>>) -> Result<Json<Vec<ResponseUser>>, ApiError> {
    let pool = data.database.pool.clone();
    let users = db_get_all_data(&pool).await?;
    Ok(Json(users))
}

pub async fn create_answer(
    answer: Json<RequestAnswer>,
    Extension(data): Extension<Arc<State>>,
) -> Result<Json<ResponseUser>, ApiError> {
    let pool = data.database.pool.clone();
    let answer: Answer = answer.0.into();
    let user = db_create_answer(&pool, &answer).await?;
    Ok(Json(user))
}

pub async fn user_sign_in(
    user: Json<SignInUser>,
    Extension(data): Extension<Arc<State>>,
    cookies: Cookies,
) -> Result<Json<ResponseUser>, ApiError> {
    let pool = data.database.pool.clone();
    let user = user.0;
    let password = db_get_user_password(&pool, &user.email).await?;
    match password.pverify(&user.password)? {
        true => {
            let user = db_get_user_data_using_email(&pool, user.email).await?;
            cookies.add(Cookie::new("session_id", user.id.reveal().to_string()));
            return Ok(Json(user));
        }
        false => Err(ApiError::InvalidPassword(
            bcrypt::BcryptError::InvalidPassword,
        )),
    }
}

pub async fn question_list(
    Extension(data): Extension<Arc<State>>,
) -> Result<Json<Vec<QuestionWithName>>, ApiError> {
    let pool = data.database.pool.clone();
    let questions = db_get_question_list(&pool).await?;
    Ok(Json(questions))
}

pub async fn get_question(
    Extension(data): Extension<Arc<State>>,
    question: Json<RequestUserQuestion>,
) -> Result<Json<QuestionWithName>, ApiError> {
    let question = question.0;
    let question_id: QuestionId = question.question_id.into();
    let pool = data.database.pool.clone();
    let question = db_get_question_by_id(&pool, &question_id).await?;

    Ok(Json(question))
}

pub async fn user_auth(
    Extension(data): Extension<Arc<State>>,
    headers: HeaderMap,
) -> Result<Json<ResponseUser>, ApiError> {
    let pool = data.database.pool.clone();

    match headers.get(COOKIE) {
        Some(data) => {
            let cookie = data.to_str().unwrap().split("=").collect::<Vec<_>>()[1];
            let user = db_get_user_data(&pool, &UserId::from(cookie.to_string())).await?;
            Ok(Json(user))
        }
        None => Err(ApiError::NoSessionCookieFound),
    }
}
