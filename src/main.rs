fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::io::Error;
    use std::time::Duration;
    use sqlx::{Connection, PgConnection, PgPool, Postgres, Row};
    use sqlx::postgres::{PgPoolOptions, PgRow};


    #[tokio::test]
    async fn test_fetch_one() -> Result<(), sqlx::Error>{
        let pool = get_pool().await?;

        let result: PgRow = sqlx::query("select * from category where id = $1")
            .bind("A")
            .fetch_one(&pool).await?;

            let id: String = result.get("id");
            let name: String = result.get("name");
            let description: String = result.get("description");
        println!("id : {}, name : {}, description : {}", id, name, description);

        Ok(())

    }
    
    #[tokio::test]
    async fn test_fetch_optional() -> Result<(), sqlx::Error> {
        let pool = get_pool().await?;
        let result: Option<PgRow> = sqlx::query("select * from category where id = $1")
            .bind("A")
            .fetch_optional(&pool).await?;

        if let Some(row) = result {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");
            println!("id: {}, name: {}, description: {}", id, name, description);
        } else {
            println!("Data is not found");
        }
        Ok(())

    }

    #[tokio::test]
    async fn test_prepare_statement() -> Result<(), sqlx::Error> {
        let pool = get_pool().await?;
        sqlx::query("insert into category (id, name, description) values ($1, $2, $3)")
            .bind("B").bind("Contoh").bind("Contoh Deskripsi")
            .execute(&pool).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_execute() -> Result<(), sqlx::Error> {
        let pool = get_pool().await?;
        sqlx::query("insert into category (id, name, description) values ('A', 'Contoh', 'Contoh')")
            .execute(&pool).await?;
        Ok(())
    }


    #[tokio::test]
    async fn test_pool_connection() -> Result<(), sqlx::Error> {
        let pool = get_pool().await?;
        pool.close().await;
        Ok(())
    }
    async fn get_pool() -> Result<PgPool, sqlx::Error> {
        let url = "postgres://postgres:postgres@localhost:5480/belajar_rust_database";
        PgPoolOptions::new()
            .max_connections(10)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(60))
            .connect(url).await
    }

    #[tokio::test]
    async fn test_manual_connection() -> Result<(), sqlx::Error> {
        let url = "postgres://postgres:postgres@localhost:5480/belajar_rust_database";
        let connection: PgConnection = PgConnection::connect(url).await?;

        connection.close().await?;
        Ok(())
    }



}
