use std::sync::Arc;
use crate::AppState;
use axum::extract::State;
use serde_json::{json, Value};

use axum::{
    Json,
    http::StatusCode,
    extract::Path,    
};

pub async fn get_userid(
    State(state): State<Arc<AppState>>,    
    Path(id): Path<i32>) -> (StatusCode, Json<Value>) {

    let mut client = state.db_client.lock().await;

    let sql = "SELECT id as userid,firstname,lastname,email,mobile,username,roles,isactivated,isblocked,mailtoken,userpic,qrcodeurl FROM users WHERE id = @p1";
    let stream = client
    .query(sql, &[&id])
    .await
    .expect("Query failed");

    let row_option: Option<tiberius::Row> = stream
    .into_row()
    .await
    .expect("Failed to get the row from the stream");

    if let Some(row) = row_option {
        let idno: i32 = row.try_get("userid").expect("ID not found in row").expect("REASON");
        let firstname: &str = row.get("firstname").expect("Missing firstname column");
        let lastname: &str = row.get("lastname").expect("Missing lastname column");
        let email: &str = row.get("email").expect("Missing email column");
        let mobile: &str = row.get("mobile").expect("Missing mobile column");
        let username: &str = row.get("username").expect("Missing username column");
        let roles: &str = row.get("roles").expect("Missing roles column");
        let isactivated: i32 = row.get("isactivated").expect("Missing isactivated column");
        let isblocked: i32 = row.get("isblocked").expect("Missing isblocked column");
        let mailtoken: i32 = row.get("mailtoken").expect("Missing mailtoken column");
        let userpic: &str = row.get("userpic").expect("Missing userpic column");
        let qrcodeurl: Option<&str> = row.get("qrcodeurl");
        let qrcodeurl_str: &str = qrcodeurl
            .unwrap_or("");

        let response = serde_json::json!({
            "id": idno,
            "firstname": firstname,
            "lastname": lastname,
            "email": email,
            "mobile": mobile,
            "username": username,
            "roles": roles,
            "isactivated": isactivated,
            "isblocked": isblocked,
            "mailtoken": mailtoken,
            "userpic": userpic,
            "qrcodeurl": qrcodeurl_str,
            "message": "User ID found.."
        });
        return (StatusCode::OK, Json(response));      

    } else {
        let response = json!({
            "message": "User ID not found.."
        });
        return (StatusCode::NOT_FOUND, Json(response));      
    }
}