use gavlib::utils::sql_utils::{SecureNote, new_sql_connection};
use log::{error, info};
use sqlx::{Row, SqliteConnection, query};
use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub struct GavDaemon {
    sql_connection: SqliteConnection,
    listener: TcpListener,
}

impl GavDaemon {
    pub async fn new(port: u16) -> anyhow::Result<Self> {
        let mut sql_connection = new_sql_connection().await?;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS NOTES (
                note_id INTEGER PRIMARY KEY AUTOINCREMENT, 
                title TEXT NOT NULL, 
                content TEXT NOT NULL
            )",
        )
        .execute(&mut sql_connection)
        .await?;

        sqlx::query(
            "INSERT INTO NOTES (title, content) 
             VALUES ('Sample Note', 'This is a sample note content.')",
        )
        .execute(&mut sql_connection)
        .await?;

        let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;
        info!("Listening on port {}", port);

        println!("SQLite database launched and 'NOTES' table is ready!");
        Ok(Self {
            sql_connection,
            listener,
        })
    }

    pub async fn run_daemon(&mut self) -> anyhow::Result<()> {
        loop {
            let (stream, addr) = self.listener.accept()?;
            info!("Connection from {}", addr);

            self.handle_client(stream).await?;
        }
    }

    async fn handle_client(&mut self, mut stream: TcpStream) -> anyhow::Result<()> {
        let peer = stream
            .peer_addr()
            .unwrap_or_else(|_| "unknown".parse().unwrap());

        let mut buffer = [0; 512];
        match stream.read(&mut buffer) {
            Ok(n) => {
                let rec_string = String::from_utf8_lossy(&buffer[..n]);
                info!("Received from {}: {}", peer, rec_string);
                info!(
                    "Inputs found while searching {}: {:?}",
                    &rec_string,
                    self.search(rec_string.to_string()).await?
                );
            },
            Err(e) => {
                error!("Failed to read from {}: {}", peer, e);
            },
        }
        Ok(())
    }

    pub async fn search(&mut self, input: String) -> anyhow::Result<Vec<SecureNote>> {
        let rows = query(
            r#"
            SELECT note_id, title, content 
            FROM NOTES
            WHERE title LIKE ? OR note_id = ?
            "#,
        )
        .bind(format!("%{}%", input)) // Use LIKE for partial matches
        .bind(input.parse::<i32>().ok()) // Bind note_id if input is numeric
        .fetch_all(&mut self.sql_connection)
        .await?;

        let results = rows
            .into_iter()
            .map(|row| SecureNote::new(row.get("title"), row.get("content"), row.get("note_id")))
            .collect();

        Ok(results)
    }
}
