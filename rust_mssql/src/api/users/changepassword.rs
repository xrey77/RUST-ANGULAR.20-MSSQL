use crate::utils; 

use std::sync::Arc;
use crate::AppState;
use axum::extract::State;
use tiberius::Query;

use axum::extract;
use axum::{
    http::StatusCode,
    extract::Path,    
    Json,
};
use serde_json::{json, Value};

use std::string::String;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct UserRequest {
    pub password: String,
}

pub async fn change_password(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    extract::Json(payload): extract::Json<UserRequest>
) -> (StatusCode, Json<Value>) {
    
    let mut client = state.db_client.lock().await;

    let sql1 = "SELECT id as userid,firstname,lastname,email,mobile,username,roles,isactivated,isblocked,mailtoken,userpic,qrcodeurl FROM users WHERE id = @p1";
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
        let hashed_password = utils::hash_password(&payload.password).unwrap();

        let sql2 = "UPDATE users SET password_digest = @p1 WHERE id = @p2";
        let mut query = Query::new(sql2);
            query.bind(&hashed_password);
            query.bind(idno);

        let execution_result = query.execute(&mut client).await.expect("Failed to execute UPDATE query");
        let _rows_affected = execution_result.rows_affected();                                 

        let response = json!({
            "message": "You have changed your password successfully."
        });
    
        return (StatusCode::OK, Json(response));

    } else {
        let response = json!({
            "message": "User ID not found."
        });
    
        return (StatusCode::NOT_FOUND, Json(response));

    }
}