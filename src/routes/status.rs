use worker::*;


pub async fn head(mut _req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let response = Response::empty()?;
    Ok(response)
}