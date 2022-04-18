
# search_wallpapers


Lambda behind API Gateway accepting optional props to either search wallpapers by name or return all, paginated or otherwise.
<br />
<hr />
<br />

## Expected Stringified Input

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSearchDto {
  pub limit: Option<i32>, // The amount you want returned
  pub start_key: Option<String>, // the primary key of an item that you want to start the query at 
  pub contains: Option<String>  // a string that's pattern matched with each qualifying records "name" attribute
}
```
<hr />
<br />

## Expected Stringified Output

```rust
#[derive(Debug, Serialize)]
pub struct DynamoImage {
  pub blurhash: String,
  pub created_at: u64,
  pub media_type: String,
  pub name: String,
  pub pk: String,
  pub sk: String,
  pub thumbnail_url: String,
  pub updated_at: u64,
  pub url: String,
}

#[derive(Serialize)]
pub struct HttpResponseBody {
  pub total: i32,
  pub images: Vec<DynamoImage>,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
struct HttpResponse {
  pub statusCode: u16,
  pub message: String,
  pub body: String, // HttpResponseBody
}
```
<hr />
<br />

## CDK Instantiation

```typescript
const search_wallpapers = new lambda.Function(this, `search_wallpapers`, {
  handler: `main`,
  runtime: lambda.Runtime.PROVIDED_AL2,
  code: lambda.Code.fromAsset(path.resolve(__dirname, `./lambdas/search_wallpapers/bootstrap.zip`)),
  functionName: `search_wallpapers`,
});
```
<hr />
<br />
