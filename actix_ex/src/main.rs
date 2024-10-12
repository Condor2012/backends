mod count;
mod error;
mod mongo;

use crate::body::BoxBody;
use crate::count::CountersTransform;
use actix_web::body;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Path};
use actix_web::{dev::Service, web, App, HttpResponse, HttpServer};
use error::CustomResult;
use futures::StreamExt;
use log::LevelFilter;
use mongo::{BoardData, MongoBoards, TaskData};
use mongodb::bson::oid::ObjectId;
use std::env;
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let mongo = MongoBoards::new(&env::var("MONGO_CONNECTION")?).await;
    let boards_data = Arc::new(mongo);

    let counters = CountersTransform::default();

    HttpServer::new(move || {
        App::new()
            .wrap(counters.clone())
            .wrap_fn(|req, srv| {
                let addr = req.peer_addr();
                log::info!("From middleware fn: Hello {addr:?}");
                srv.call(req)
            })
            .app_data(Data::new(boards_data.clone()))
            .service(create_board)
            .service(read_boards)
            .service(read_board)
            .service(update_board)
            .service(delete_board)
            .service(create_task)
            .service(read_tasks)
            .service(read_task)
            .service(update_task)
            .service(delete_task)
            .default_service(web::to(default_response))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}

async fn default_response() -> CustomResult<HttpResponse> {
    Ok(HttpResponse::Ok().body("Go to '/board'"))
}

#[actix_web::post("/board")]
async fn create_board(
    board_data: web::Json<BoardData>,
    boards: web::Data<Arc<MongoBoards>>,
) -> CustomResult<HttpResponse> {
    let data = board_data.into_inner();
    let created = boards.create_board(data).await?;
    Ok(HttpResponse::Ok().json(created))
}

#[actix_web::get("/board")]
async fn read_boards(boards: web::Data<Arc<MongoBoards>>) -> CustomResult<HttpResponse> {
    let boards = boards.read_boards().await?;
    Ok(HttpResponse::Ok().json(boards))
}

#[actix_web::get("/board/{id}")]
async fn read_board(
    path: Path<String>,
    boards: web::Data<Arc<MongoBoards>>,
) -> CustomResult<HttpResponse> {
    let id = ObjectId::from_str(&path.into_inner())?;
    let boards = boards.read_board(id).await?;
    Ok(HttpResponse::Ok().json(boards))
}

#[actix_web::put("/board/{id}")]
async fn update_board(
    path: Path<String>,
    board_data: web::Json<BoardData>,
    boards: web::Data<Arc<MongoBoards>>,
) -> CustomResult<HttpResponse> {
    let data = board_data.into_inner();
    let id = ObjectId::from_str(&path.into_inner())?;
    let updated = boards.update_board(id, data).await?;
    Ok(HttpResponse::Ok().json(updated))
}

#[actix_web::delete("/board/{id}")]
async fn delete_board(
    path: Path<String>,
    boards: web::Data<Arc<MongoBoards>>,
) -> CustomResult<HttpResponse> {
    let id = ObjectId::from_str(&path.into_inner())?;
    let deleted = boards.delete_board(id).await?;
    Ok(HttpResponse::Ok().json(deleted))
}

#[actix_web::post("/board/{id}/task")]
async fn create_task(
    path: Path<String>,
    task_data: web::Json<TaskData>,
    boards: web::Data<Arc<MongoBoards>>,
) -> CustomResult<HttpResponse> {
    let data = task_data.into_inner();
    let id = ObjectId::from_str(&path.into_inner())?;
    let created = boards.create_task(id, &data).await?;
    Ok(HttpResponse::Ok().json(created))
}

#[actix_web::get("/board/{id}/task")]
async fn read_tasks(
    path: Path<String>,
    boards: web::Data<Arc<MongoBoards>>,
) -> CustomResult<HttpResponse> {
    let id = ObjectId::from_str(&path.into_inner())?;
    let tasks = boards.read_tasks(id).await?;
    Ok(HttpResponse::Ok().json(tasks))
}

#[actix_web::get("/board/{id}/task/{name}")]
async fn read_task(
    path: Path<(String, String)>,
    boards: web::Data<Arc<MongoBoards>>,
) -> CustomResult<HttpResponse> {
    let (id, name) = path.into_inner();
    let id = ObjectId::from_str(&id)?;
    let task = boards.read_task(id, &name).await?;
    Ok(HttpResponse::Ok().json(task))
}

#[actix_web::put("/board/{id}/task/{name}")]
async fn update_task(
    path: Path<(String, String)>,
    task_data: web::Json<TaskData>,
    boards: web::Data<Arc<MongoBoards>>,
) -> CustomResult<HttpResponse> {
    let data = task_data.into_inner();
    let (id, name) = path.into_inner();
    let id = ObjectId::from_str(&id)?;
    let updated = boards.update_task(id, &name, &data).await?;
    Ok(HttpResponse::Ok().json(updated))
}

#[actix_web::delete("/board/{id}/task/{name}")]
async fn delete_task(
    path: Path<(String, String)>,
    boards: web::Data<Arc<MongoBoards>>,
) -> CustomResult<HttpResponse> {
    let (id, name) = path.into_inner();
    let id = ObjectId::from_str(&id)?;
    let deleted = boards.delete_task(id, &name).await?;
    Ok(HttpResponse::Ok().json(deleted))
}
