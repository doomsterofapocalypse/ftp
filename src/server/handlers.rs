use axum::{
    response::{Html,IntoResponse},
    extract::Multipart,
    http::StatusCode,
};


const SAVE_FILE_BASE_PATH: &str = ".";
const DOWNLOAD_DIR_PATH:&str = ".";


pub async fn show_upload() -> Html<&'static str> {
    Html(include_str!("./static/uploads.html"))
}


pub async fn save_file(mut multipart: Multipart)-> Result<impl IntoResponse, StatusCode>{

    if  let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let filename = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        println!("[+] Length of `{}` is {} bytes, content type is:{}, filename is: {}", name, data.len(), content_type, filename);
        println!("[+] writing file to: {}/{}", SAVE_FILE_BASE_PATH, filename);
        let save_filename = format!("{}/{}", SAVE_FILE_BASE_PATH, filename);
        
        let uploadres = tokio::fs::write(&save_filename, &data).await;
        match uploadres {
            Ok(_) => {
                Ok(StatusCode::CREATED)
            }
            Err(e) => {
                println!("[!!] upload error:{}", e);
                Ok(StatusCode::BAD_REQUEST)
            }
        }
    }
    else{
        println!("[!!] There is no file uploaded or the file format is incorrect ");
        Ok(StatusCode::BAD_REQUEST)
    }
}