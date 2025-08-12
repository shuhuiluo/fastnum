use fastnum::*;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:example@localhost")
        .await?;

    let row: (UD128, D128) = sqlx::query_as("SELECT $1, $2")
        .bind(udec128!(3.14159265358979323846264))
        .bind(dec128!(-3.14159265358979323846264))
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.0, udec128!(3.14159265358979323846264));
    assert_eq!(row.1, dec128!(-3.14159265358979323846264));

    sqlx::query("DROP TABLE IF EXISTS test_d256")
        .execute(&pool)
        .await
        .ok();
    sqlx::query("CREATE TABLE IF NOT EXISTS test_d256 (value DECIMAL)")
        .execute(&pool)
        .await?;

    let value: UD256 = udec256!(1) / udec256!(3);
    sqlx::query("INSERT INTO test_d256 (value) VALUES ($1)")
        .bind(value)
        .execute(&pool)
        .await?;

    let result: UD256 = sqlx::query_scalar("SELECT value FROM test_d256")
        .fetch_one(&pool)
        .await?;
    println!("Result: {}", result);

    // Test with real value
    sqlx::query("DELETE FROM test_d256").execute(&pool).await?;

    let value: UD256 = udec256!(99999888254.14037123881);
    sqlx::query("INSERT INTO test_d256 (value) VALUES ($1)")
        .bind(value)
        .execute(&pool)
        .await?;

    let result: UD256 = sqlx::query_scalar("SELECT value FROM test_d256")
        .fetch_one(&pool)
        .await?;

    println!("Result: {}", result);

    Ok(())
}
