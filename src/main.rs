fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use futures::TryStreamExt;
    use sqlx::{Connection, Error, PgConnection, PgPool, Pool, Postgres, Row};
    use sqlx::postgres::{PgPoolOptions, PgRow};
    use sqlx::FromRow;
    use chrono::{Local, NaiveDateTime};

    #[tokio::test]
    async fn test_auto_increment_with_transaction() -> Result<(), sqlx::Error> {
        let pool = get_pool().await?;
        let mut transaction = pool.begin().await?;

        sqlx::query("insert into sellers(name) values ($1) returning id;")
            .bind("Contoh Seller")
            .execute(&mut *transaction).await?;

        let result: PgRow = sqlx::query("select lastval() as id")
            .fetch_one(&mut *transaction).await?;

        transaction.commit().await?;

        let id: i32 = result.get_unchecked("id");
        println!("Id Seller : {}", id);

        Ok(())
    }

    #[tokio::test]
    async fn test_auto_increment() -> Result<(), sqlx::Error> {
        let pool = get_pool().await?;

        let result : PgRow = sqlx::query("insert into  sellers(name) values ($1) returning id;")
            .bind("Contoh seller")
            .fetch_one(&pool).await?;

        let id: i32 = result.get("id");
        println!("Id Seller : {}", id);

        Ok(())
    }

    #[tokio::test]
    async fn test_transaction() -> Result<(), sqlx::Error> {
        let pool = get_pool().await?;
        let mut transaction = pool.begin().await?;


        sqlx::query("insert into brands (id, name, description, created_at, updated_at) values ($1, $2, $3, $4, $5);")
            .bind("B")
            .bind("Contoh Name B")
            .bind("Contoh Description B")
            .bind(Local::now().naive_local())
            .bind(Local::now().naive_local())
            .execute(&mut *transaction).await?;


        sqlx::query("insert into brands (id, name, description, created_at, updated_at) values ($1, $2, $3, $4, $5);")
            .bind("D")
            .bind("Contoh Name D")
            .bind("Contoh Description D")
            .bind(Local::now().naive_local())
            .bind(Local::now().naive_local())
            .execute(&mut *transaction).await?;


        sqlx::query("insert into brands (id, name, description, created_at, updated_at) values ($1, $2, $3, $4, $5);")
            .bind("C")
            .bind("Contoh Name C")
            .bind("Contoh Description C")
            .bind(Local::now().naive_local())
            .bind(Local::now().naive_local())
            .execute(&mut *transaction).await?;

        transaction.commit().await?;
        Ok(())
    }



    #[tokio::test]
    async fn test_result_mapping_brand() -> Result<(), sqlx::Error> {
        let pool = get_pool().await?;

        let result : Vec<Brand> = sqlx::query_as("select * from brands")
            .fetch_all(&pool).await?;

        for brand in result {
            println!("{:?}", brand);
        }

        Ok(())
    }





    #[tokio::test]
    async fn test_insert_brand() -> Result<(), sqlx::Error> {
        let pool = get_pool().await?;

        sqlx::query("insert into brands (id, name, description, created_at, updated_at) values ($1, $2, $3, $4, $5);")
            .bind("a")
            .bind("Contoh Name")
            .bind("Contoh Description")
            .bind(Local::now().naive_local())
            .bind(Local::now().naive_local())
            .execute(&pool).await?;
        Ok(())
    }


    #[derive(FromRow, Debug)]
    struct Brand {
        id: String,
        name: String,
        description: String,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    }


    #[tokio::test]
    async fn test_result_mapping_automatic() -> Result<(), sqlx::Error> {
        let pool = get_pool().await?;

        let result: Vec<Category> = sqlx::query_as("select * from category")
            .fetch_all(&pool).await?;

        for category in result {
            println!("{:?}", category);
        }

        Ok(())
    }



    #[tokio::test]
    async fn test_result_mapping() ->  Result<(), sqlx::Error>{
        let pool = get_pool().await?;

        let mut result = sqlx::query("select * from category")
            .map(|row: PgRow| {
                Category {
                    id: row.get("id"),
                    name: row.get("name"),
                    description: row.get("description"),
                }
            })
            .fetch_all(&pool).await?;

        for category in result  {
            println!("{:#?}", category);
        }
        Ok(())
    }



    #[derive(Debug, FromRow)]
    struct Category {
        id : String,
        name: String,
        description: String,
    }

    #[tokio::test]
    async fn test_fetch() ->  Result<(), sqlx::Error>{
        let pool = get_pool().await?;

        let mut result = sqlx::query("select * from category")
            .fetch(&pool);

        while let Some(row) = result.try_next().await? {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");
            println!("id : {}, name : {}, description : {}", id, name, description);
        }

        Ok(())
    }


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
