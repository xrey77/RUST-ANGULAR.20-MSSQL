use std::sync::Arc;
use crate::AppState;
use axum::extract::State;
use tiberius::Query;

use axum::{ 
    extract::{Multipart},
};
use serde_json::{json, Value};
use axum::{
    http::StatusCode,
    extract::Path,    
    Json,
};

use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::info;
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Users {
//     userpic: String,
// }

pub async fn patch_uploadpicture(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>, mut multipart: Multipart) -> (StatusCode, Json<Value>) {

        while let Some(field) = multipart.next_field().await.unwrap() {

            let mut client = state.db_client.lock().await;
            let sql1 = "SELECT id as userid, userpic FROM users WHERE id = @p1";
            let stream = client
            .query(sql1, &[&id])
            .await
            .expect("Query failed");
        
            let row_option: Option<tiberius::Row> = stream
            .into_row()
            .await
            .expect("Failed to get the row from the stream");
        
            if let Some(row) = row_option {
                    let idno: i32 = row.try_get("userid").expect("ID not found in row").expect("REASON");
                    let userpic: &str = row.get("userpic").expect("Missing userpic column");

                    let name = field.name().unwrap().to_string();
                    let file_name = field.file_name().unwrap().to_string(); //originam filename
                    let content_type = field.content_type().unwrap().to_string();
                    let data = field.bytes().await.unwrap();

                    let extension = std::path::Path::new(&file_name)
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("jpg"); 

                    let new_filename = format!("00{}.{}", idno, extension);

                    info!(
                        "Name: {}, FileName: {}, ContentType: {}, Size: {} bytes",
                        name, file_name, content_type, data.len()
                    );

                    // Check if the field is an image (optional)
                    if content_type.starts_with("image/") {
                        let path = format!("./assets/users/{}", new_filename);
                        
                        // delete old picture
                        if userpic != "pix.png" {
                            let oldpic = format!("./assets/users/{}", userpic);
                            let _ = fs::remove_file(oldpic).await;
                        }
                        // Ensure the 'uploads' directory exists
                        if let Err(e) = tokio::fs::create_dir_all("./assets").await {
                            eprintln!("Failed to create directory: {}", e);

                            let response = json!({
                                "message": "Failed to create upload directory."
                            });
                            return (StatusCode::CONFLICT, Json(response));

                        }

                        // Write the file to disk asynchronously
                        let mut file = match File::create(&path).await {
                            Ok(file) => file,
                            Err(e) => {
                                eprintln!("Failed to create file: {}", e);

                                let response = json!({
                                    "message": "Failed to create file."
                                });
                                return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
                            }
                        };

                        if let Err(e) = file.write_all(&data).await {
                            eprintln!("Failed to write to file: {}", e);

                            let response = json!({
                                "message": "Failed to create file."
                            });
                            return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
                        }

                        let sql2 = "UPDATE users SET userpic = @p1 WHERE id = @p2";
                        let mut query = Query::new(sql2);
                            query.bind(&new_filename);
                            query.bind(idno);
                
                        let execution_result = query.execute(&mut client).await.expect("Failed to execute UPDATE query");
                        let _rows_affected = execution_result.rows_affected();                                 

                        let response = json!({
                            "userpic": new_filename,
                            "message": "You have change your profile picture successfully."
                        });
                    
                        return (StatusCode::OK, Json(response));
            

                    } else {

                            let response = json!({
                                "message": "Invalid file type. Only images are allowed."
                            });
                            return (StatusCode::BAD_REQUEST, Json(response));

                    }
                    
            
            } else {

                let response = json!({
                    "message": "User ID not found."
                });
            
                return (StatusCode::NOT_FOUND, Json(response));
        
            }    
        }

        let response = json!({
            "message": "No file uploaded."
        });
        return (StatusCode::BAD_REQUEST, Json(response));
 }