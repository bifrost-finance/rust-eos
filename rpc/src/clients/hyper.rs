use crate::error::Error;
use crate::client::Client;
use hyper::rt::{Future, Stream};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug)]
pub struct HyperClient {
    node: String,
}

impl HyperClient {
    pub fn new(node: &str) -> Self {
        Self {
            node: node.to_owned(),
        }
    }
}

impl Client for HyperClient {
    fn node(&self) -> &str {
        &self.node
    }

    fn fetch<T>(&self, path: impl AsRef<str>, params: impl Serialize) -> crate::Result<T>
        where T: 'static + for<'b> Deserialize<'b> + Send + Sync
    {
        let https = HttpsConnector::new(4)?;
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);
        let url = self.node.to_string() + path.as_ref();
        let url: hyper::Uri = url.parse()?;

        let json = serde_json::to_string(&params)?;
        let mut req = hyper::Request::new(hyper::Body::from(json));
        *req.method_mut() = hyper::Method::POST;
        *req.uri_mut() = url;
        req.headers_mut().insert(
            hyper::header::CONTENT_TYPE,
            hyper::header::HeaderValue::from_static("application/json"),
        );
        req.headers_mut().insert(
            hyper::header::ACCEPT,
            hyper::header::HeaderValue::from_static("application/json"),
        );

        let fut = client
            .request(req)
            .and_then(|res| res.into_body().concat2())
            .from_err::<Error>()
            .and_then(|body| {
                let block: T = serde_json::from_slice(&body)?;
                Ok(block)
            });
        let result = tokio::runtime::Runtime::new()?.block_on(fut)?;
        Ok(result)
    }
}
