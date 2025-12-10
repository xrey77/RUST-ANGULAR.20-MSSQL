use std::sync::Arc;
use crate::AppState;
use axum::extract::State;
use base32::Alphabet;
use serde_json::{json, Value};

use axum::{
    http::StatusCode,
    extract::Path,    
    Json,
};

use totp_rs::{Algorithm, TOTP};
use std::time::SystemTime;

pub async fn patch_verifytotp(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>, req: String) -> (StatusCode, Json<Value>) {

    let json_value: serde_json::Value = serde_json::from_str(&req).unwrap();
    let otpcode = &json_value["otp"];
    let mut client = state.db_client.lock().await;
    let sql = "SELECT username, email, secret FROM users WHERE id = @p1";
    let stream = client
    .query(sql, &[&id])
    .await
    .expect("Query failed");

    let row_option: Option<tiberius::Row> = stream
    .into_row()
    .await
    .expect("Failed to get the row from the stream");

    if let Some(row) = row_option {

        let username: &str = row.get("username").expect("Missing username column");
        let email: &str = row.get("email").expect("Missing email column");
        let secret: &str = row.get("secret").expect("Missing secret column");

        let secret_bytes = match base32::decode(Alphabet::Rfc4648 { padding: true }, &secret) {
            Some(bytes) => bytes,
            None => {
                let response = json!({
                    "message": "Internal server error: Invalid secret format."
                });    
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
            }
        };
    
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,       // Digits
            1,       // Allowed drift
            30,      // Period
            secret_bytes,       
            Some("APPLE INC.".to_string()),
            email.to_string(),
        ).unwrap();
    
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
    
        if totp.check(&otpcode.to_string(), time) {
    
            let response = json!({
                "username": username,
                "message": "OTP code has verified successfully."
            });    
            return (StatusCode::OK, Json(response))        
            
        } else {
    
            let response = json!({
                "message": "Invalid OTP code, please try again."
            });    
            return (StatusCode::CONFLICT, Json(response))    
            
        }    

    } else {

        let response = json!({
            "message": "User ID not found."
        });    
        return (StatusCode::CONFLICT, Json(response))    

    }

}