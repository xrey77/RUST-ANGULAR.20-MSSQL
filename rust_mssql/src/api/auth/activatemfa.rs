use std::sync::Arc;
use crate::AppState;
use axum::extract::State;

use axum::{
    http::StatusCode,
    extract::Path,    
    Json,
};
use tiberius::Query;
use data_encoding::BASE32;
use std::string::String;
use serde_json::{json, Value};
use totp_rs::{Algorithm, TOTP, Secret};

// #[axum::debug_handler]
pub async fn patch_activatemfa(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>, req: String) -> (StatusCode, Json<Value>) {
    
    let mut client = state.db_client.lock().await;

    let json_value: serde_json::Value = serde_json::from_str(&req).unwrap();
    let is_mfaenable = &json_value["TwoFactorEnabled"];

    if is_mfaenable == true {
        let sql1 = "SELECT email FROM users WHERE id = @p1";

        let stream = client
        .query(sql1, &[&id])
        .await
        .expect("Query failed");
    
        let row_option: Option<tiberius::Row> = stream
        .into_row()
        .await
        .expect("Failed to get the row from the stream");
    
        if let Some(row) = row_option {

            let email: &str = row.get("email").expect("Missing email column");
            let secret: Secret = Secret::generate_secret();

            let totp = TOTP::new(
                Algorithm::SHA1,
                6,
                1,
                30,
                secret.to_bytes().expect("Invalid secret bytes"),
                Some("APPLE INC.".to_string()),
                email.to_string(),
            ).unwrap();        

            let qrcode_base64 = totp.get_qr_base64();
            let qrcode_string: String = qrcode_base64.clone().expect("Failed to get the base64 string");

            let secret_bytes: Vec<u8> = secret.to_bytes().expect("Failed to convert secret to bytes");
            let encoded_secret: String = BASE32.encode(&secret_bytes);
        
            let sql2 = "UPDATE users SET qrcodeurl = @p1, secret = @p2 WHERE id = @p3";
            let mut query = Query::new(sql2);
                query.bind(&qrcode_string);
                query.bind(&encoded_secret);
                query.bind(id);
 
            let execution_result = query.execute(&mut client).await.expect("Failed to execute UPDATE query");
            let _rows_affected = execution_result.rows_affected();                                 

            let response = json!({
                "qrcodeurl": &qrcode_string,
                "message": format!("Multi-Factor Authenticator has been anabled." )
            });    
            return (StatusCode::OK, Json(response))

        } else {

            let response = json!({
                "message": "User ID not found."
            });

            return (StatusCode::NOT_FOUND, Json(response));
        }

    } else {

        let sql2 = "UPDATE users SET qrcodeurl = null, secret = null WHERE id = @p1";
        let mut query = Query::new(sql2);
            query.bind(id);

        let execution_result = query.execute(&mut client).await.expect("Failed to execute UPDATE query");
        let _rows_affected = execution_result.rows_affected();                                 

        let response = json!({
            "message": "Multi-Factor Authenticator has been disabled."
        });    

        return (StatusCode::OK, Json(response));
    }
}