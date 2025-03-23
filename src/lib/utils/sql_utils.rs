use sqlx::{Connection, MySqlConnection};

pub struct SecureNote {
    title: String,
    content: String,
    note_id: i32,
}

impl SecureNote {
    pub fn new(title: String, content: String, note_id: i32) -> Self {
        Self {
            title,
            content,
            note_id,
        }
    }
}

pub async fn new_sql_connection() -> anyhow::Result<MySqlConnection> {
    Ok(MySqlConnection::connect("sqlite::memory:").await?)
}

/*
//#[tokio::main]
//pub async fn foobar() -> Result<(), sqlx::Error> {
pub async fn foobar() -> anyhow::Result<()> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await?;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
    let row: (i64,) = sqlx::query_as("SELECT ?1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, 150);

    Ok(())
}
*/
//
