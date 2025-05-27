use worker::*;

const TEMPLATE : &str = include_str!("../public/html/index.html");

pub async fn get(mut _req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::from_html(TEMPLATE)
}