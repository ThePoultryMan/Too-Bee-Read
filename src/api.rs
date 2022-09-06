use reqwest::Error;

pub async fn search_for_book(name: &str) -> Result<(), Error> {
    let body = reqwest::get("https://www.googleapis.com/books/v1/volumes?q=".to_owned() + name)
        .await?
        .text()
        .await?;

    println!("{}", body);

    Ok(())
}
