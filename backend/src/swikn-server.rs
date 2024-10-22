use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use askama::Template;
use tower_http::services::ServeDir;

use axum::{
    extract,
    response::{Html, IntoResponse, Response},
};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/users", post(create_user))
        .nest_service("/static", ServeDir::new("static"))
        .route("/greet/:name", get(greet))
        .route("/:tool", get(tool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    // axum_server::bind(&"0.0.0.0:3000".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
async fn tool(extract::Path(tool): extract::Path<String>) -> impl IntoResponse {
    Html(swikn::html::html_tool(&tool))
}

async fn greet(extract::Path(name): extract::Path<String>) -> impl IntoResponse {
    let template = HelloTemplate { name };
    HtmlTemplate(template)
}

async fn index() -> impl IntoResponse {
    let template = IndexTemplate { tools: &["a"] };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    tools: &'a [&'a str],
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
