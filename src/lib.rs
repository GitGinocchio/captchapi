use worker::*;

pub mod routes;
use crate::routes::index;
use crate::routes::captcha;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let router = Router::new()
        .get_async("/", index::get)
        .get_async("/captcha/show", captcha::get)
        .get_async("/captcha", captcha::get);

    router.run(req, env).await
}