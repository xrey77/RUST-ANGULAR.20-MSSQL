use std::sync::Arc;
use crate::AppState;
use axum::extract::State;

use axum::{
    http::StatusCode,
    Json,
};
use serde::{Serialize};
use serde_json::{json, Value};

use tiberius::Query;

#[derive(Debug, Serialize)]
pub struct Users {
    id: i32,
    firstname: String,
    lastname: String,
    email: String,
    mobile: String,
    username: String,
    roles: String,
    isactivated: i32,
    isblocked: i32,
}

pub async fn get_allusers(
    State(state): State<Arc<AppState>>) -> (StatusCode, Json<Value>) {
        let mut client = state.db_client.lock().await;
        let sql = "SELECT id,firstname,lastname,email,mobile,username,roles,isactivated,isblocked FROM users";

        let query = Query::new(sql);


        let rows = query
        .query(&mut client)
        .await
        .expect("Failed to execute query")
        .into_first_result() // Collects the output of the first query into a Vec<Row>
        .await
        .expect("Failed to collect rows");        

        let users: Vec<Users> = rows
        .into_iter()
        .map(|row| {
            Users {
                id: row.get::<i32, _>("id").expect("id not found"),
                firstname: row.get::<&str, _>("firstname").map(String::from).expect("firstname not found"),
                lastname: row.get::<&str, _>("lastname").map(String::from).expect("lastname not found"),
                email: row.get::<&str, _>("email").map(String::from).expect("email not found"),
                mobile: row.get::<&str, _>("mobile").map(String::from).expect("mobile not found"),
                username: row.get::<&str, _>("username").map(String::from).expect("username not found"),
                roles: row.get::<&str, _>("roles").map(String::from).expect("roles not found"),
                isactivated: row.get::<i32, _>("isactivated").expect("isactivated not found"),
                isblocked: row.get::<i32, _>("isblocked").expect("isblocked not found")
            }
        })
        .collect();


        let response = json!({
            "users": users,
            "message": "User records found.."
        });
    
        return (StatusCode::OK, Json(response));

}