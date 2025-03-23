use gavlib::utils::sql_utils::{new_sql_connection, SecureNote};
use sqlx::{Connection, MySqlConnection};

pub struct GavDaemon {
    sql_connection: MySqlConnection,
}

impl GavDaemon {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            sql_connection: new_sql_connection().await?,
        })
    }
    pub async fn search(&mut self, input:String) -> Vec<SecureNote>{
        sqlx::query_as ("SELECT * FROM NOTES WHERE title = ?1 OR note_id = ?1")
            .bind(input)
            .fetch_all(&mut self.sql_connection)
            .await
            .into_iter()
            .collect::<>()
        }

}
