# ignore_image_toggle

Lambda to toggle an images `ignore` status in DynamoDB.  Images are marked as ignored on the GUI, which means they will remain in S3 but its metadata will be hidden from most API calls.  Ex -- The `url` of a saved image dies and is breaking the GUI, etc.