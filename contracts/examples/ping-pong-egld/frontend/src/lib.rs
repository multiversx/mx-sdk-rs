use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub async fn tx_request(endpoint: &str, body: &str) -> Result<JsValue, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&JsValue::from_str(body));

    let headers = Headers::new().unwrap();
    headers.set("Content-Type", "application/json").unwrap();
    opts.set_headers(&headers);

    let url = format!("http://localhost:8000/tx/{}", endpoint);
    let req = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window = web_sys::window().unwrap();

    let resp_value = JsFuture::from(window.fetch_with_request(&req)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}

#[wasm_bindgen]
pub async fn query_request(endpoint: &str) -> Result<JsValue, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let url = format!("http://localhost:8000/query/{}", endpoint);
    let req = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&req)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}
