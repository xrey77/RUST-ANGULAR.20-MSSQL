use std::sync::Arc;
use crate::AppState;
use axum::extract::State;

use axum::{
    http::StatusCode,
    Json,
};
use tiberius::{Client, Row};
use tokio::net::TcpStream;
use tokio_util::compat::Compat;
use anyhow::Result;

use axum::{ extract::Path,};
use serde_json::{json, Value};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct Products {
    id: i32,
    category: String,
    descriptions: String,
    qty: i32,
    unit: String,
    costprice: f64,
    sellprice: f64,
    saleprice: f64,
    productpicture: String,
    alertstocks: i32,
    criticalstocks: i32
}

#[derive(Deserialize)]
pub struct Params {
    page: i32,
    key: String,
}


async fn get_products(client: &mut Client<Compat<TcpStream>>, offset: i32, per_page: i32, searchkey: &str) -> Result<Vec<Products>> {
    let sql = "SELECT id, category, descriptions, qty, unit, costprice, sellprice, saleprice, productpicture, alertstocks, criticalstocks FROM products WHERE LOWER(descriptions) LIKE @P1 ORDER BY id OFFSET @P2 ROWS FETCH NEXT @P3 ROWS ONLY;";
    let search_pattern = format!("%{}%", searchkey.to_lowercase());
    let rows = client
        .query(sql, &[&search_pattern, &offset, &per_page])
        .await?
        .into_results()
        .await?
        .pop()
        .unwrap_or_default();

    let mut products = Vec::new();

    for row in rows {
        let category: &str = row.get(1).ok_or(anyhow::anyhow!("Missing category")).expect("REASON");
        let descriptions: &str = row.get(2).ok_or(anyhow::anyhow!("Missing descriptions")).expect("REASON");
        let qty: i32 = row.get(3).ok_or(anyhow::anyhow!("Missing qty")).expect("REASON");
        let unit: &str = row.get(4).ok_or(anyhow::anyhow!("Missing unit")).expect("REASON");

        let costprice_numeric: tiberius::numeric::Numeric = row.get(5).ok_or(anyhow::anyhow!("Missing sellprice")).expect("REASON");
        let sellprice_numeric: tiberius::numeric::Numeric = row.get(6).ok_or(anyhow::anyhow!("Missing sellprice")).expect("REASON");
        let saleprice_numeric: tiberius::numeric::Numeric = row.get(7).ok_or(anyhow::anyhow!("Missing saleprice")).expect("REASON");
    
        let cost: f64 = f64::from(costprice_numeric);
        let sell: f64 = f64::from(sellprice_numeric);
        let sale: f64 = f64::from(saleprice_numeric);

        let prodpicture: &str = row.get(8).ok_or(anyhow::anyhow!("Missing productpictre")).expect("REASON");
        let alert: i32 = row.get(9).ok_or(anyhow::anyhow!("Missing alertstocks")).expect("REASON");
        let critical: i32 = row.get(9).ok_or(anyhow::anyhow!("Missing criticalstocks")).expect("REASON");
        
        let product = Products {
            id: row.get(0).unwrap_or_default(),
            category: category.to_string(),
            descriptions: descriptions.to_string(),
            qty: qty,
            unit: unit.to_string(),
            costprice: cost,
            sellprice: sell,
            saleprice: sale,
            productpicture: prodpicture.to_string(),
            alertstocks: alert,
            criticalstocks: critical
        };
        products.push(product);
    }

    Ok(products)
}

async fn get_product_count(client: &mut Client<Compat<TcpStream>>, searchkey: &str) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {

    let search_pattern = format!("%{}%", searchkey.to_lowercase());
    let row_stream = client.query("SELECT COUNT(*) AS count FROM products WHERE LOWER(descriptions) LIKE @P1", &[&search_pattern]).await?;

    let row: Row = row_stream
        .into_row()
        .await?
        .expect("Expected a single row with the count");

    let totalrecords_count: i32 = row.get("count").expect("Column 'count' not found");
    Ok(totalrecords_count)
}


pub async fn get_productsearch(
    State(state): State<Arc<AppState>>,
    Path(params): Path<Params>) -> (StatusCode, Json<Value>) {

    let page =params.page as i32;
    let search = params.key.to_string();

    let mut client = state.db_client.lock().await;
    let totalrecords_count_result = get_product_count(&mut client, &search).await;
    let totalrecords_count = totalrecords_count_result.expect("Failed to get product count");
    
    let per_page: i32 = 5;
    let offset: i32 = (page - 1) * per_page;
    let total1: f64 = totalrecords_count as f64 / per_page as f64;
    let totpages = total1.ceil();
    let total_pages = totpages as u32;    
    
    let products = get_products(&mut client, offset, per_page, &search).await.expect("REASON");

    if products.is_empty() {
        let response = json!({
            "message": "Product not found."
        });
        return (StatusCode::NOT_FOUND, Json(response));       
    }


    let response = json!({
        "page": page,
        "totpage": total_pages,
        "totalrecords": totalrecords_count,
        "products": products,
        "message": "Product records has been retrieved."
    });

    return (StatusCode::OK, Json(response));   
}