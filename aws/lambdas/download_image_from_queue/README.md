# download_wallpaper_from_queue

Lambda triggered when an item is placed onto the Download Wallpaper SQS Queue.  Takes item off the queue, writes the metadata to DynamoDB and saves image to S3.  Deletes from queue -- this should be the last process run on any wallpaper being saved.