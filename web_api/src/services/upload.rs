use axum::{extract::Multipart, response::IntoResponse};
use chrono::Utc;
use dotenv::dotenv;
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn upload(mut multipart: Multipart) -> impl IntoResponse {
    dotenv().ok();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let time = Utc::now()
            .to_string()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("")
            .replace(['.', ':', '-'], "")
            .replace("UTC", "");
        let data = field.bytes().await.unwrap();
        let path = dotenv::var("UPLOAD_PATH").unwrap();
        println!("{}", name);
        match name.as_str() {
            "nfe" => {
                let file_name = format!("{}/nfe/{}.xml", path, time);
                let mut file = File::create(&file_name).await.unwrap();
                file.write_all(&data).await.unwrap();
            }
            _ => {
                // Handle other field names here
            }
        }
    }
}
