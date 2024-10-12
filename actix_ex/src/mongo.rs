use crate::error::{CustomError, CustomResult};
use crate::{ObjectId, StreamExt};
use mongodb::bson::{doc, ser};
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct BoardData {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    tasks: Vec<TaskData>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TaskData {
    name: String,
    description: String,
}

#[derive(Clone)]
pub struct MongoBoards(Client);

impl MongoBoards {
    pub async fn new(connection_str: &str) -> Self {
        Self(Client::with_uri_str(connection_str).await.unwrap())
    }

    pub async fn create_board(&self, data: BoardData) -> CustomResult<BoardData> {
        let collection = self.0.database("boards_db").collection("boards");
        let inserted = collection.insert_one(data, None).await?;
        let id = inserted.inserted_id;
        let query = doc! { "_id": &id };
        let board = collection.find_one(query, None).await?;
        board.ok_or_else(|| CustomError::NotFound(format!("board with id: {}", id)))
    }

    pub async fn read_boards(&self) -> CustomResult<Vec<BoardData>> {
        let collection = self.0.database("boards_db").collection("boards");
        let query = doc! {};
        let mut boards = collection.find(query, None).await?;

        let mut boards_vec = Vec::new();
        while let Some(board) = boards.next().await {
            boards_vec.push(board?);
        }

        Ok(boards_vec)
    }

    pub async fn read_board(&self, id: ObjectId) -> CustomResult<BoardData> {
        let collection = self.0.database("boards_db").collection("boards");
        let query = doc! { "_id": &id };
        let board = collection.find_one(query, None).await?;
        board.ok_or_else(|| CustomError::NotFound(format!("board with id: {}", id)))
    }

    pub async fn update_board(&self, id: ObjectId, mut data: BoardData) -> CustomResult<BoardData> {
        data.id = None;
        let collection: Collection<BoardData> = self.0.database("boards_db").collection("boards");
        let query = doc! { "_id": &id };
        let update = doc! { "$set": ser::to_bson(&data)? };
        collection.update_one(query, update, None).await?;
        self.read_board(id).await
    }

    pub async fn delete_board(&self, id: ObjectId) -> CustomResult<BoardData> {
        let collection = self.0.database("boards_db").collection("boards");
        let query = doc! { "_id": &id };
        let board = collection.find_one_and_delete(query, None).await?;
        board.ok_or_else(|| CustomError::NotFound(format!("board with id: {}", id)))
    }

    pub async fn create_task(&self, id: ObjectId, data: &TaskData) -> CustomResult<TaskData> {
        let collection: Collection<BoardData> = self.0.database("boards_db").collection("boards");
        let query = doc! { "_id": &id };
        let update = doc! { "$push": {"tasks": ser::to_bson(data)? } };
        collection.update_one(query, update, None).await?;
        self.read_task(id, &data.name).await
    }

    pub async fn read_tasks(&self, id: ObjectId) -> CustomResult<Vec<TaskData>> {
        self.read_board(id).await.map(|b| b.tasks)
    }

    pub async fn read_task(&self, id: ObjectId, name: &str) -> CustomResult<TaskData> {
        let board = self.read_board(id).await?;
        let task = board.tasks.into_iter().find(|t| t.name == name);
        task.ok_or_else(|| CustomError::NotFound(format!("task with name: {}", name)))
    }

    pub async fn delete_task(&self, id: ObjectId, name: &str) -> CustomResult<TaskData> {
        let task = self.read_task(id, name).await?;
        let collection: Collection<BoardData> = self.0.database("boards_db").collection("boards");
        let query = doc! { "_id": &id };
        let update = doc! { "$pull": {"tasks": {"name": name} } };
        collection.update_one(query, update, None).await?;
        Ok(task)
    }

    pub async fn update_task(
        &self,
        id: ObjectId,
        name: &str,
        data: &TaskData,
    ) -> CustomResult<TaskData> {
        self.delete_task(id, name).await?;
        self.create_task(id, data).await
    }
}
