use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> { 
    // mini-redis 와 연결한다.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // key: hello, value: world 로 입력
    client.set("hello", "world".into()).await?;

    // hello 키로 값 얻기
    let result = client.get("hello").await?;

    println!("서버로부터 값을 받았습니다: {:?}", result);
    // 서버로부터 값을 받았습니다: Some(b"world")

    Ok(())
}
