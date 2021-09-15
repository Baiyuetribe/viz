use std::{future::Future, pin::Pin};

use viz_core::{http, Context, Middleware, Response, Result};

/// Cookies Middleware
#[derive(Debug, Default)]
pub struct Cookies {}

impl Cookies {
    async fn run(&self, cx: &mut Context) -> Result<Response> {
        cx.cookies()?;

        let mut res = cx.next().await?;

        let cookies = cx.cookies()?;

        let jar = cookies.read();

        for cookie in jar.delta() {
            res.headers_mut().insert(
                http::header::SET_COOKIE,
                http::header::HeaderValue::from_str(&cookie.encoded().to_string())?,
            );
        }

        Ok(res)
    }
}

impl<'a> Middleware<'a, Context> for Cookies {
    type Output = Result<Response>;

    #[must_use]
    fn call(
        &'a self,
        cx: &'a mut Context,
    ) -> Pin<Box<dyn Future<Output = Self::Output> + Send + 'a>> {
        Box::pin(self.run(cx))
    }
}
