use worker::*;

pub mod utils;
pub mod routes;
use crate::routes::index;
use crate::routes::captcha;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let router = Router::new()
        .get_async("/", index::get)
        //.get_async("/captcha/show", generate::get)
        .get_async("/captcha", captcha::fetch)
        .get_async("/captcha.:format", captcha::fetch);

    router.run(req, env).await
}