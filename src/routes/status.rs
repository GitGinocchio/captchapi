use worker::*;


pub async fn head(mut _req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    match Response::empty() {
        Ok(response) => {
            Ok(response.with_status(200))
        }
        Err(e) => {
            Err(e)
        }
    }
}