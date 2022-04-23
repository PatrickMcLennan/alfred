use serde::Serialize;

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
pub struct ResponseHeaders {
  #[serde(rename(serialize = "Set-Cookie"))]
  pub Set_Cookie: Vec<String>,
}

// Implement Display for the Failure response so that we can then implement Error.
impl std::fmt::Display for Response {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", self.body)
  }
}

// Implement Error for the Response so that we can `?` (try) the Response
// returned by `lambda_runtime::run(func).await` in `fn main`.
impl std::error::Error for Response {}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Response {
  pub statusCode: u16,
  pub body: String,
  pub multiValueHeaders: Option<ResponseHeaders>,
  pub headers: Option<ResponseHeaders>
}
