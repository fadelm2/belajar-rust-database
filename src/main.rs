fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use sqlx::{Connection, PgConnection};



    #[tokio::test]
    async fn test_manual_connection() -> Result<(), sqlx::Error> {
        let url = "postgres://postgres:postgres@localhost:5480/belajar_rust_database";
        let connection: PgConnection = PgConnection::connect(url).await?;

        connection.close().await?;
        Ok(())
    }

}
