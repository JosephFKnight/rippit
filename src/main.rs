use rippit::Reddit;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut r = Reddit::new("".to_owned(), None);
    r.connect().await;
    println!("{:#?}", r.get_hot().await?);
    Ok(())
}

