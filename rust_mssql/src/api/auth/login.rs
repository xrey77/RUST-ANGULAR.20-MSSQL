use crate::utils; 

use std::sync::Arc;
use crate::AppState;
use axum::extract::State;

use serde::{Deserialize, Serialize};
use axum::{
    http::StatusCode,
    Json,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestData {
    username: String,
    password: String
}

use serde_json::{json, Value};

pub async fn create_login(
    State(state): State<Arc<AppState>>,
    payload: Json<RequestData>) -> (StatusCode, Json<Value>) {

    let mut client = state.db_client.lock().await;

    let sql = "SELECT id as userid, firstname, lastname, email, mobile, username, password_digest, roles, isactivated, isblocked, userpic, qrcodeurl FROM users WHERE username = @p1";

    let stream = client
    .query(sql, &[&payload.username])
    .await
    .expect("Query failed");

    let row_option: Option<tiberius::Row> = stream
    .into_row()
    .await
    .expect("Failed to get the row from the stream");

    if let Some(row) = row_option {
            let idno: i32 = row.try_get("userid").expect("ID not found in row").expect("REASON");
            let username: &str = row.get("username").expect("Missing username column");
            let password_digest: Option<&str> = row.get("password_digest");    
            let firstname: &str = row.get("firstname").expect("Missing firstname column");
            let lastname: &str = row.get("lastname").expect("Missing lastname column");
            let email: &str = row.get("email").expect("Missing email column");
            let mobile: &str = row.get("mobile").expect("Missing mobile column");
            let roles: &str = row.get("roles").expect("Missing roles column");
            let isactivated: i32 = row.get("isactivated").expect("Missing isactivated column");
            let isblocked: i32 = row.get("isblocked").expect("Missing isblocked column");
            let userpic: &str = row.get("userpic").expect("Missing userpic column");
            let qrcodeurl: Option<&str> = row.get("qrcodeurl");
            let qrcodeurl_str: &str = qrcodeurl
                .unwrap_or("");

            match utils::verify_password(&payload.password, &password_digest.unwrap()) {
                Ok(is_valid) => {
                    if is_valid {
            
                        let mut token_string = String::new();
                        let secret_key = std::env::var("JWT_SECRET").expect("JWT_SECRET not set").as_bytes().to_vec();                        
                        let jwt_token = utils::create_jwt(&email, &secret_key);
                        match jwt_token {
                            Ok(token) => {
                                token_string.push_str(&token);
                            }
                            Err(e) => {
                                eprintln!("Failed to create JWT : {}", e);
                            }
                        }
                        
                        let response = json!({
                            "id": idno,
                            "firstname": firstname,
                            "lastname": lastname,
                            "email": email,
                            "mobile": mobile,
                            "username": username,
                            "roles": roles,
                            "isactivated": isactivated,
                            "isblocked": isblocked,
                            "userpic": userpic,
                            "qrcodeurl": qrcodeurl_str,
                            "token": token_string,
                            "message": "Logged-In Successful."
                        });
                    
                        return (StatusCode::OK, Json(response));
            
                    } else {
                        let response = json!({
                            "message": "Invalid password."
                        });
            
                        return (StatusCode::CONFLICT, Json(response));
                    }
                }
                Err(e) => {
            
                    let response = json!({
                        "message": e.to_string()
                    });
                
                    return (StatusCode::CONFLICT, Json(response));
            
                }
            }

    } else {

        let response = json!({
            "message": "Username not found, please register."
        });

        (StatusCode::NOT_FOUND, Json(response))

    }
}






