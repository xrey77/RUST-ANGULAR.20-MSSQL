use crate::utils; 

use std::sync::Arc;
use crate::AppState;
use axum::extract::State;

use axum::extract;
use axum::{
    http::StatusCode,
    Json,
};
use tiberius::{Row, Query};
use serde_json::{json, Value};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct UserRequest {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub mobile: String,
    pub username: String,
    pub password: String,
}

#[axum::debug_handler]
pub async fn create_user(
    State(state): State<Arc<AppState>>,    
    extract::Json(payload): extract::Json<UserRequest>
) -> (StatusCode, Json<Value>) {
    let mut client = state.db_client.lock().await;


    let sql1 = "SELECT COUNT(*) FROM users WHERE email = @P1";

    let stream1 = client
        .query(sql1, &[&payload.email])
        .await.expect("REASON");    

    let row_option1: Option<Row> = stream1.into_row().await.expect("Expected a row");     

    // EMAIL VALIDATION
    match row_option1 {
        Some(row) => {
            let count: i32 = row.get(0).unwrap_or(0); 
            if count > 0 {

                let response = json!({
                    "message": "Email Address is already taken."
                });            
                return (StatusCode::CONFLICT, Json(response))        

            } else {
                // USERNAME VALIDATION
                let sql2 = "SELECT COUNT(*) FROM users WHERE username = @P1";
                let stream2 = client
                .query(sql2, &[&payload.username])
                .await.expect("REASON");    
        
                let row_option2: Option<Row> = stream2.into_row().await.expect("Expected a row");     
        
                match row_option2 {
                    Some(row) => {
                        let count: i32 = row.get(0).unwrap_or(0); 
                        if count > 0 {
            
                            let response = json!({
                                "message": "Username is already taken."
                            });            
                            return (StatusCode::CONFLICT, Json(response))        
            
                        } else {
                        
                            // INSERT INPUT DATA
                            let hashed_password = utils::hash_password(&payload.password).unwrap();
                            let sql3 = "INSERT INTO users (firstname,lastname,email,mobile,username,password_digest) 
                            VALUES (@p1, @p2, @p3, @p4, @p5, @p6)"; 
                            let mut query = Query::new(sql3);
                                query.bind(&payload.firstname);
                                query.bind(&payload.lastname);
                                query.bind(&payload.email);
                                query.bind(&payload.mobile);
                                query.bind(&payload.username);
                                query.bind(&hashed_password);

                            let execution_result = query.execute(&mut client).await.expect("Failed to execute INSERT query");
                            let rows_inserted = execution_result.rows_affected();                                 
                            let response = json!({
                                "message": format!("You have registered successfully, your User ID is {:?}", rows_inserted)
                            });            
                            return (StatusCode::CONFLICT, Json(response))        
            
                        }
                    }
                    None => {
                        let response = json!({
                            "message": "No count."
                        });            
                        return (StatusCode::CONFLICT, Json(response))        
            
                    }
                }



            }
        }
        None => {
            let response = json!({
                "message": "No count."
            });            
            return (StatusCode::CONFLICT, Json(response))        

        }
    }




}