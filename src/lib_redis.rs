extern crate redis;

#[tokio::main]
pub async fn connect_redis(key: String, data: String) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut conn = client.get_async_connection().await?;

    redis::cmd("SET")
        .arg(&[&key, &data])
        .query_async(&mut conn)
        .await?;

    redis::cmd("EXPIRE")
        .arg(&key)
        .arg(86400)
        .query_async(&mut conn)
        .await?;

    Ok(())
}