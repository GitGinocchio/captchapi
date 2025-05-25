use worker::*;

pub async fn get(mut _req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let body = r#"Captcha api routes:
    /captcha            # Generate a captcha image and returns the image in base64 and the text
    /captcha/show       # Generate a captcha image and shows it in the browser"#;
    Response::ok(body)
}