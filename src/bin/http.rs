use api::database::Database;
use api::State;
use axum::extract::Extension;
use axum::http::header::CONTENT_TYPE;
use axum::http::Method;
use axum::routing::{get, post};
use axum::{Router};

use api::http::web::routes::*;
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{CorsLayer, Origin};
#[tokio::main]
async fn main() {
    let state = Arc::new(State {
        database: Database::new("postgres://postgres:@localhost:5432/rhelpnet").await,
    });

    let app = Router::new()
        .route("/", get(home))
        .route("/api/new/user", post(create_user))
        .route("/api/new/question", post(create_question))
        .route("/api/new/answer", post(create_answer))
        .route("/api/signin", post(user_sign_in))
        .route("/api/questions", get(question_list))
        .route("/api/get-question", post(get_question))
        .route("/api/auth", get(user_auth))
        .route("/api/get-user", post(get_user_data_using_id))
        .layer(Extension(state))
        .layer(CookieManagerLayer::new())
        .layer(
            CorsLayer::new()
                .allow_origin(Origin::exact("http://localhost:4200".parse().unwrap()))
                .allow_methods(vec![Method::GET, Method::POST])
                .allow_credentials(true)
                .allow_headers(vec![CONTENT_TYPE]),
        );
    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
