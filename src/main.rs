mod db_module;
mod template_base;
use askama::Template;
use axum::{
    extract::{Form, Extension}, // Import State here
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Router,
};
use rusqlite::{Connection, Result};
use db_module::{get_cafes, Cafe};
use template_base::CafeListTemplate;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

#[tokio::main]
async fn main() -> Result<()> {

    // Create a shared database connection wrapped in Arc and Mutex
    let conn = Arc::new(Mutex::new(Connection::open("my_database.db")?));

    // build our application with a route
    let app = Router::new()
        .route("/", get(show_cafes))
        .route("/add", post(add_task))
        .layer(Extension(conn.clone()));

    // Define the address for the server to bind to
    let addr = "127.0.0.1:3000".parse().unwrap();
    println!("Listening on {}", addr);

    // Run the server
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();


    Ok(())
}
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}



async fn show_cafes(Extension(conn): Extension<Arc<Mutex<Connection>>>,) -> Html<String> {
    let conn = conn.lock().unwrap(); // Lock the connection for thread-safety
    // fetch cafes using the module
    let cafes = get_cafes(&conn).unwrap();
    let template = CafeListTemplate { cafes: &cafes };
    Html(template.render().unwrap())
}
// Handler to add a new task
async fn add_task(Form(input): Form<AddTask>) -> StatusCode {
    //let mut tasks = TASKS.lock().unwrap();
    //tasks.push(input.task);
    StatusCode::SEE_OTHER
}

#[derive(serde::Deserialize)]
struct AddTask {
    task: String,
}