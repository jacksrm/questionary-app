use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    application::activity::{error::ActivityError, repository::ActivityRepository},
    domain::activity::Activity,
};

pub struct SqliteActivityRepository {
    pool: SqlitePool,
}

impl SqliteActivityRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ActivityRepository for SqliteActivityRepository {
    async fn save(activity: &Activity) -> Result<(), ActivityError> {
        todo!()
    }

    async fn delete(id: &Uuid) -> Result<Activity, ActivityError> {
        todo!()
    }

    async fn get_all() -> Result<Vec<Activity>, ActivityError> {
        todo!()
    }

    async fn find(id: &Uuid) -> Result<Option<Activity>, ActivityError> {
        todo!()
    }
}

#[cfg(test)]
mod test;
