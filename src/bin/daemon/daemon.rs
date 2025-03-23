use gavlib::utils::sql_utils::{SecureNote, new_sql_connection};
use sqlx::{MySqlConnection, query, Row};

pub struct GavDaemon {
    sql_connection: MySqlConnection,
}

impl GavDaemon {
    pub async fn new() -> anyhow::Result<Self> {
    let mut connection = new_sql_connection().await?;
        sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)")
        .execute(&mut connection)
        .await?;

    println!("SQLite database launched and 'users' table is ready!");
        Ok(Self {
            sql_connection: connection,
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

