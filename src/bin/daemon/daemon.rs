use gavlib::utils::sql_utils::{SecureNote, new_sql_connection};
use sqlx::{MySqlConnection, query, Row};

pub struct GavDaemon {
    sql_connection: MySqlConnection,
}

impl GavDaemon {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            sql_connection: new_sql_connection().await?,
        })
    }

    pub async fn search(&mut self, input: String) -> anyhow::Result<Vec<SecureNote>> {
        let rows = query(
            r#"
            SELECT * FROM NOTES
            WHERE title = ? OR note_id = ?
            "#
        )
        .bind(&input)
        .bind(&input)
        .fetch_all(&mut self.sql_connection)
        .await?;

        // Map rows to SecureNote, depending on its structure
        let results = rows
            .into_iter()
            .map(|row| SecureNote::new(
                    row.get("title"),
                    row.get("content"),
                    row.get("note_id"),
            ))
            .collect();

        Ok(results)
    }
}

