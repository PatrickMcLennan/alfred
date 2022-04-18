
# search_wallpapers


Lambda that assigns an httpsOnly JWT cookie to the callers document if Cognito approves of the credentials provided.
<br />
<hr />
<br />

## Expected Stringified Input

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginDto {
  pub email: Option<String>, 
  pub password: Option<String>,
}
```
<hr />
<br />

## Expected Stringified Output

```rust
#[derive(Serialize)]
pub struct HttpResponseBody {
  pub success: bool,
  pub message: String,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
struct HttpResponse {
  pub statusCode: u16,
  pub body: String,
}
```
<hr />
<br />

## CDK Instantiation

```typescript
const login = new lambda.Function(this, `login`, {
  handler: `main`,
  runtime: lambda.Runtime.PROVIDED_AL2,
  code: lambda.Code.fromAsset(path.resolve(__dirname, `./lambdas/login/bootstrap.zip`)),
  functionName: `login`,
});
```
<hr />
<br />