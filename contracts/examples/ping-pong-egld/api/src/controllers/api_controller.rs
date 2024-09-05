use actix_web::*;
use serde_json::*;

pub async fn tx(path: web::Path<String>, body: web::Json<Value>) -> impl Responder {
    let tx_type = path.into_inner();
    let client = reqwest::Client::new();

    println!("Request: {:?}", body);

    let res = client
        .post(format!("http://localhost:8002/{}", tx_type))
        .json(&body)
        .send()
        .await
        .unwrap();

    println!("Response: {:?}", res);
    let json: Value = res.json().await.unwrap();
    HttpResponse::Ok().json(json)
}

pub async fn query(path: web::Path<String>) -> impl Responder {
    let query_type = path.into_inner();
    let client = reqwest::Client::new();

    let res = client
        .get(format!("http://localhost:8001/{}", query_type))
        .send()
        .await
        .unwrap();

    let json: Value = res.json().await.unwrap();
    HttpResponse::Ok().json(json)
}
