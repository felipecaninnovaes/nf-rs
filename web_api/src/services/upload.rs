use axum::extract::Multipart;
use chrono::Utc;
use dotenv::dotenv;
use tokio::{fs::File, io::AsyncWriteExt};


pub async fn upload(mut multipart: Multipart) {
    dotenv().ok();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        if name == "file" {
            let time = Utc::now()
                .to_string()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join("")
                .replace(['.', ':', '-'], "")
                .replace("UTC", "");
            let data = field.bytes().await.unwrap();
            let path = dotenv::var("UPLOAD_PATH").unwrap();
            let file_name = format!("{}/{}.xml", path, time);
            let mut file = File::create(file_name).await.unwrap();
            file.write_all(&data).await.unwrap();
        }
    }
}