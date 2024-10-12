use crate::{BoardData, CustomResult, TaskData};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

#[derive(Clone)]
pub struct Boards {
    pool: Pool<MySql>,
}

impl Boards {
    pub async fn new() -> Self {
        let conn_str = std::env::var("DATABASE_URL").unwrap();
        let pool = MySqlPoolOptions::new()
            .max_connections(10)
            .connect(&conn_str)
            .await
            .unwrap();
        Self { pool }
    }

    pub async fn create_board(&self, data: &BoardData) -> CustomResult<BoardData> {
        let id = sqlx::query!("INSERT INTO boards (name) VALUES(?)", &data.name)
            .execute(&self.pool)
            .await?
            .last_insert_id();

        self.read_board(id).await
    }

    pub async fn read_boards(&self) -> CustomResult<Vec<BoardData>> {
        let rows = sqlx::query!("SELECT * FROM boards")
            .fetch_all(&self.pool)
            .await?;

        let boards = rows
            .into_iter()
            .map(|item| BoardData {
                id: Some(item.id.to_string()),
                name: item.name,
                tasks: Default::default(),
            })
            .collect();

        Ok(boards)
    }

    pub async fn read_board(&self, id: u64) -> CustomResult<BoardData> {
        let row = sqlx::query!("SELECT name FROM boards WHERE id=?", id)
            .fetch_one(&self.pool)
            .await?;

        Ok(BoardData {
            id: Some(id.to_string()),
            name: row.name,
            tasks: Default::default(),
        })
    }

    pub async fn delete_board(&self, id: u64) -> CustomResult<BoardData> {
        let board = self.read_board(id).await?;
        sqlx::query!("DELETE FROM boards WHERE id=?", id)
            .execute(&self.pool)
            .await?;

        Ok(board)
    }

    pub async fn update_board(&self, id: u64, data: &BoardData) -> CustomResult<BoardData> {
        sqlx::query!("UPDATE boards SET name=? WHERE id=?", &data.name, id)
            .execute(&self.pool)
            .await?;

        self.read_board(id).await
    }

    pub async fn create_task(&self, id: u64, data: &TaskData) -> CustomResult<TaskData> {
        sqlx::query!(
            "INSERT INTO tasks (board_id, name, description) VALUES(?, ?, ?)",
            id,
            data.name,
            data.description
        )
        .execute(&self.pool)
        .await?;

        self.read_task(id, &data.name).await
    }

    pub async fn read_tasks(&self, id: u64) -> CustomResult<Vec<TaskData>> {
        let rows = sqlx::query!("SELECT * FROM tasks WHERE board_id=?", id)
            .fetch_all(&self.pool)
            .await?;

        let tasks = rows
            .into_iter()
            .map(|row| TaskData {
                name: row.name,
                description: row.description,
            })
            .collect();

        Ok(tasks)
    }

    pub async fn read_task(&self, id: u64, name: &str) -> CustomResult<TaskData> {
        let row = sqlx::query!("SELECT * FROM tasks WHERE board_id=? AND name=?", id, name)
            .fetch_one(&self.pool)
            .await?;

        Ok(TaskData {
            name: row.name,
            description: row.description,
        })
    }

    pub async fn update_task(
        &self,
        id: u64,
        name: &str,
        data: &TaskData,
    ) -> CustomResult<TaskData> {
        sqlx::query!(
            "UPDATE tasks SET name=?, description=? WHERE board_id=? AND name=?",
            data.name,
            data.description,
            id,
            name
        )
        .execute(&self.pool)
        .await?;

        self.read_task(id, &data.name).await
    }

    pub async fn delete_task(&self, id: u64, name: &str) -> CustomResult<TaskData> {
        let task = self.read_task(id, name).await?;
        sqlx::query!("DELETE FROM tasks WHERE board_id=? AND name=?", id, name)
            .execute(&self.pool)
            .await?;

        Ok(task)
    }
}
