
# attach_blurhash

Lambda triggered when an item is placed onto the Blurhash SQS Queue.  Takes item off the queue, creates + attaches a blurhash to the items metadata and then places item into another queue of the callers choice.

## Expected Input

```rust

struct Payload<T> {
  pub s
}

```