extern crate redis;
use redis::AsyncCommands;

#[tokio::main]
async fn connect_redis(key: String, data: String) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut conn = client.get_async_connection().await?;

    conn.set("data", b"test").await?;
    redis::cmd("SET")
        .arg(&["key2", "bar"])
        .query_async(&mut conn)
        .await?;

    let result = redis::cmd("MGET")
        .arg(&["data", "key2"])
        .query_async(&mut conn)
        .await?;

    Ok(())
}