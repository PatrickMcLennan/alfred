
# attach_blurhash


Takes stringified image off of the `blurhashQueue`, generates + attaches a [blurhash](https://blurha.sh/) made from it's `thumbnail_url` metadata, stringify's + places new metadata with blurhash onto `downloadImageQueue`.
<br />
<hr />
<br />

## Expected Stringified Input

```rust
#[derive(Serialize, Deserialize)]
pub struct BlurhashQueueInputItem {
  pub name: String,
  pub url: String,
  pub thumbnail_url: String,  
}
```
<hr />
<br />

## Expected Stringified Output

```rust
#[derive(Serialize, Deserialize)]
pub struct BlurhashQueueOutputItem {
  pub name: String,
  pub url: String,
  pub thumbnail_url: String,  
  pub blurhash: String,  
}
```
<hr />
<br />

## CDK Instantiation

```typescript
const attach_blurhash = new lambda.Function(this, `attach_blurhash`, {
  handler: `main`,
  runtime: lambda.Runtime.PROVIDED_AL2,
  code: lambda.Code.fromAsset(path.resolve(__dirname, `./lambdas/attach_blurhash/bootstrap.zip`)),
  functionName: `attach_blurhash`,
});
attach_blurhash.addEventSource(
  new SqsEventSource(blurhashQueue, {
    batchSize: 1,
  })
);
```
<hr />
<br />
