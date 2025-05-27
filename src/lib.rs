use worker::*;

pub mod routes;
use crate::routes::index;
use crate::routes::generate;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let router = Router::new()
        .get_async("/", index::get)
        //.get_async("/captcha/show", generate::get)
        .post_async("/generate", generate::post);

    router.run(req, env).await
}