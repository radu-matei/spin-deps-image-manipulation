use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

mod bindings;

use bindings::deps::component::image_manipulation_lib::image_manipulation;

/// A simple Spin HTTP component.
#[http_component]
fn handle_image_manipulation_http_rs(req: Request) -> anyhow::Result<impl IntoResponse> {
    let img = image_manipulation::grayscale(&req.body().to_vec(), 100)?;
    Ok(Response::builder()
        .status(200)
        .header("content-type", "image/jpeg")
        .body(img)
        .build())
}
