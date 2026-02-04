use actix_files::Files as ActixFiles;
use actix_web::{
    App, HttpResponse, HttpServer, Result as ActixResult, error::ErrorInternalServerError, get,
    post, web,
};
use askama::Template;
use repository::Repository;

mod repository;
mod templates;

// Feel free to delete or modify this route. This is just an example.
#[get("/")]
async fn index(repo: web::Data<Repository>) -> ActixResult<HttpResponse> {
    // This is how you get all the todos from the database.
    let todos = repo.get_all().await.map_err(ErrorInternalServerError)?;

    // Example from documentation:
    // let context = MyTemplate { data: some_vec };
    let template = templates::IndexTemplate { todos };
    let body = template.render().map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

// Example from documentation:
// #[derive(serde::Deserialize)]
// struct MyForm { name: String }
//
// #[post("/route")]
// async fn my_handler(form: web::Form<MyForm>) -> HttpResponse { ... }
#[post("/add")]
async fn add_todo(
    repo: web::Data<Repository>,
    form: web::Form<TodoForm>,
) -> ActixResult<HttpResponse> {
    repo.insert(form.todo_name.clone())
        .await
        .map_err(ErrorInternalServerError)?;
    //repo.get_all().await.map_err(ErrorInternalServerError)?;
    let repo_list = repo.get_all().await.map_err(ErrorInternalServerError)?;
    let template = templates::TodoListTemplate { todos: repo_list };
    let body = template.render().map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(body))
}

#[derive(serde::Deserialize)]
struct TodoForm {
    todo_name: String,
}

#[derive(serde::Deserialize)]
struct HelloForm {
    name: String,
}

#[post("/hello")]
async fn hello(user_input: web::Form<HelloForm>) -> ActixResult<HttpResponse> {
    let template = templates::HelloTemplate {
        name: user_input.name.clone(),
    };

    let body = template.render().map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/delete_all")]
async fn delete_all(repo: web::Data<Repository>) -> ActixResult<HttpResponse> {
    repo.delete_all_done()
        .await
        .map_err(ErrorInternalServerError)?;
    let template = templates::TodoListTemplate {
        todos: repo.get_all().await.map_err(ErrorInternalServerError)?,
    };
    let body = template.render().map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(body))
}

#[post("/delete_everything")]
async fn delete_everything(repo: web::Data<Repository>) -> ActixResult<HttpResponse> {
    repo.delete_everything()
        .await
        .map_err(ErrorInternalServerError)?;
    let template = templates::TodoListTemplate {
        todos: repo.get_all().await.map_err(ErrorInternalServerError)?,
    };
    let body = template.render().map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(body))
}

#[post("/delete/{id}")]
async fn delete_todo(repo: web::Data<Repository>, id: web::Path<i64>) -> ActixResult<HttpResponse> {
    repo.delete(id.into_inner())
        .await
        .map_err(ErrorInternalServerError)?;
    let template = templates::TodoListTemplate {
        todos: repo.get_all().await.map_err(ErrorInternalServerError)?,
    };
    let body = template.render().map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(body))
}

#[post("/toggle/{id}")]
async fn toggle_doto(repo: web::Data<Repository>, id: web::Path<i64>) -> ActixResult<HttpResponse> {
    let mut found_id = repo
        .get_by_id(id.into_inner())
        .await
        .map_err(ErrorInternalServerError)?;
    found_id.is_done = !found_id.is_done;
    repo.update(found_id)
        .await
        .map_err(ErrorInternalServerError)?;

    let template = templates::TodoListTemplate {
        todos: repo.get_all().await.map_err(ErrorInternalServerError)?,
    };
    let body = template.render().map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/edit/{id}")]
async fn edit_todo(
    repo: web::Data<Repository>,
    id: web::Path<i64>,
    req: actix_web::HttpRequest,
) -> ActixResult<HttpResponse> {
    let new_text = req
        .headers()
        .get("HX-Prompt")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    if !new_text.is_empty() {
        let mut todo = repo
            .get_by_id(id.into_inner())
            .await
            .map_err(ErrorInternalServerError)?;
        todo.text = new_text.to_string();
        repo.update(todo).await.map_err(ErrorInternalServerError)?;
    }

    let template = templates::TodoListTemplate {
        todos: repo.get_all().await.map_err(ErrorInternalServerError)?,
    };
    let body = template.render().map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const PORT: u16 = 8080;
    const HOST: &str = "127.0.0.1";

    dotenv::dotenv().ok();
    let database = Repository::try_init()
        .await
        .expect("Failed to initialize database, contact the author.");

    println!("Starting server at http://{}:{}", HOST, PORT);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            // If you're going to add new routes, don't forget to add them here!
            .service(index)
            .service(hello)
            .service(add_todo)
            .service(toggle_doto)
            .service(delete_all)
            .service(delete_everything)
            .service(delete_todo)
            .service(edit_todo)
            // This is how you serve css files from the static folder.
            .service(ActixFiles::new("/", "./src/static").prefer_utf8(true))
    })
    .bind((HOST, PORT))?
    .run()
    .await
}
