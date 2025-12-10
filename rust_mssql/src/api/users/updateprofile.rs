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

use std::string::String;
use serde::{Deserialize};
use serde_json::{json, Value};

#[derive(Deserialize, Debug)]
pub struct UserRequest {
    pub firstname: String,
    pub lastname: String,
    pub mobile: String,
}

// #[axum::debug_handler]
pub async fn patch_updateprofile(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    extract::Json(payload): extract::Json<UserRequest>
) -> (StatusCode, Json<Value>) {
    
    let mut client = state.db_client.lock().await;

    let sql1 = "SELECT id as userid FROM users WHERE id = @p1";
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

        let sql2 = "UPDATE users SET firstname = @p1, lastname = @p2, mobile = @p3 WHERE id = @p4";
        let mut query = Query::new(sql2);
            query.bind(&payload.firstname);
            query.bind(&payload.lastname);
            query.bind(&payload.mobile);
            query.bind(idno);

        let execution_result = query.execute(&mut client).await.expect("Failed to execute UPDATE query");
        let _rows_affected = execution_result.rows_affected();                                 

        let response = json!({
            "message": "You have updated your profile successfully."
        });
    
        return (StatusCode::OK, Json(response));

    } else {
        let response = json!({
            "message": "User ID not found."
        });
    
        return (StatusCode::NOT_FOUND, Json(response));

    }

}