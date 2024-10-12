use crate::counter::CounterMiddleware;
use crate::error::CustomResult;
use crate::mysq::Boards;
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::State;

mod counter;
mod error;
mod mysq;

#[rocket::launch]
async fn rocket() -> _ {
    dotenv::dotenv().unwrap();

    let db = Boards::new().await;

    rocket::build()
        .manage(db)
        .attach(AdHoc::on_request("Hello mw", |req, _| {
            Box::pin(async move {
                let addr = req
                    .client_ip()
                    .map(|addr| addr.to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                log::info!("Hi, {addr}.");
            })
        }))
        .attach(CounterMiddleware::default())
        .mount(
            "/board",
            rocket::routes![
                create_board,
                read_board,
                read_boards,
                delete_board,
                update_board,
                create_task,
                read_task,
                read_tasks,
                delete_task,
                update_task,
            ],
        )
}

#[rocket::post("/", data = "<data>")]
async fn create_board(data: Json<BoardData>, db: &State<Boards>) -> Json<CustomResult<BoardData>> {
    db.create_board(&data).await.into()
}

#[rocket::get("/")]
async fn read_boards(db: &State<Boards>) -> Json<CustomResult<Vec<BoardData>>> {
    db.read_boards().await.into()
}

#[rocket::get("/<id>")]
async fn read_board(id: u64, db: &State<Boards>) -> Json<CustomResult<BoardData>> {
    db.read_board(id).await.into()
}

#[rocket::put("/<id>", data = "<data>")]
async fn update_board(
    id: u64,
    data: Json<BoardData>,
    db: &State<Boards>,
) -> Json<CustomResult<BoardData>> {
    db.update_board(id, &data).await.into()
}

#[rocket::delete("/<id>")]
async fn delete_board(id: u64, db: &State<Boards>) -> Json<CustomResult<BoardData>> {
    db.delete_board(id).await.into()
}

#[rocket::post("/<id>/task", data = "<data>")]
async fn create_task(
    id: u64,
    data: Json<TaskData>,
    db: &State<Boards>,
) -> Json<CustomResult<TaskData>> {
    db.create_task(id, &data).await.into()
}

#[rocket::get("/<id>/task")]
async fn read_tasks(id: u64, db: &State<Boards>) -> Json<CustomResult<Vec<TaskData>>> {
    db.read_tasks(id).await.into()
}

#[rocket::get("/<id>/task/<name>")]
async fn read_task(id: u64, name: &str, db: &State<Boards>) -> Json<CustomResult<TaskData>> {
    db.read_task(id, name).await.into()
}

#[rocket::put("/<id>/task/<name>", data = "<data>")]
async fn update_task(
    id: u64,
    name: &str,
    data: Json<TaskData>,
    db: &State<Boards>,
) -> Json<CustomResult<TaskData>> {
    db.update_task(id, name, &data).await.into()
}

#[rocket::delete("/<id>/task/<name>")]
async fn delete_task(id: u64, name: &str, db: &State<Boards>) -> Json<CustomResult<TaskData>> {
    db.delete_task(id, name).await.into()
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct BoardData {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    name: String,
    tasks: Vec<TaskData>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TaskData {
    name: String,
    description: String,
}
