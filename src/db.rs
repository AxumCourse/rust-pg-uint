use crate::model;

pub async fn insert(conn: &sqlx::PgPool, m: &model::Test) -> Result<model::Test, sqlx::Error> {
    let id = xid::new().to_string();
    sqlx::query_as("INSERT INTO test (id, num) VALUES ($1, $2) RETURNING *")
        .bind(&id)
        .bind(m.num)
        .fetch_one(conn)
        .await
}

pub async fn find(
    conn: &sqlx::PgPool,
    num: model::Uint,
) -> Result<Option<model::Test>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM test WHERE num=$1 LIMIT 1")
        .bind(num)
        .fetch_optional(conn)
        .await
}

pub async fn list(conn: &sqlx::PgPool) -> Result<Vec<model::Test>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM test").fetch_all(conn).await
}

#[cfg(test)]
mod test {
    use crate::model;

    async fn get_conn() -> sqlx::PgPool {
        let dsn = std::env::var("PG_DSN")
            .unwrap_or("postgres://postgres:postgres@127.0.0.1:5432/draft".to_string());
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&dsn)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_insert() {
        let conn = get_conn().await;
        let i = model::Uint::new(1);
        let r = super::insert(
            &conn,
            &model::Test {
                num: i,
                ..Default::default()
            },
        )
        .await
        .unwrap();
        assert!(r.num.value() == i.value());
        assert!(r.num.usinged() == i.usinged());
    }

    #[tokio::test]
    async fn test_u32_max_insert() {
        let conn = get_conn().await;
        let i = model::Uint::from(u32::MAX);
        let r = super::insert(
            &conn,
            &model::Test {
                num: i,
                ..Default::default()
            },
        )
        .await
        .unwrap();
        assert!(r.num.value() == i.value());
        assert!(r.num.usinged() == i.usinged());
        println!("{} {}", r.num.value(), r.num.usinged());
    }

    #[tokio::test]
    async fn test_i32_min_insert() {
        let conn = get_conn().await;
        let i = model::Uint::from(i32::MIN);
        let r = super::insert(
            &conn,
            &model::Test {
                num: i,
                ..Default::default()
            },
        )
        .await
        .unwrap();
        assert!(r.num.value() == i.value());
        assert!(r.num.usinged() == i.usinged());
        println!("{} {}", r.num.value(), r.num.usinged());
    }

    #[tokio::test]
    async fn test_find() {
        let conn = get_conn().await;
        let i = model::Uint::from(i32::MIN);
        let r = super::find(&conn, i).await.unwrap();
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(r.num.value() == i.value());
        assert!(r.num.usinged() == i.usinged());
        println!("{} {}", r.num.value(), r.num.usinged());
    }

    #[tokio::test]
    async fn test_list() {
        let conn = get_conn().await;
        let ls = super::list(&conn).await.unwrap();
        for r in ls.iter() {
            println!("{:?} - {}", r, r.num.usinged());
        }
    }
}
