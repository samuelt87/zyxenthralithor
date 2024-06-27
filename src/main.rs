use axum::{
    extract::Form,
    response::Html,
    routing::{get, post},
    Router,
};
use rusqlite::{params, Connection};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}

async fn show_form() -> Html<&'static str> {
    Html(include_str!("index.html"))
}

async fn submit_form(Form(form_data): Form<FormData>) -> Html<&'static str> {
    let conn = Connection::open("contacts.sqlite").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS contacts (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )",
        [],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO contacts (name, email) VALUES (?1, ?2)",
        params![form_data.name, form_data.email],
    )
    .unwrap();
    Html("Thank you for your submission!")
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(show_form))
        // `POST /users` goes to `create_user`
        .route("/submit", post(submit_form));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
