import * as cdk from '@aws-cdk/core';
import * as lambda from '@aws-cdk/aws-lambda';
import * as s3 from '@aws-cdk/aws-s3';
import * as sqs from '@aws-cdk/aws-sqs';
import * as path from 'path';
import * as dynamo from '@aws-cdk/aws-dynamodb';
import * as events from '@aws-cdk/aws-events';
import * as targets from '@aws-cdk/aws-events-targets';
import { SqsEventSource } from '@aws-cdk/aws-lambda-event-sources';

import { config } from 'dotenv';

config({ path: path.resolve(__dirname, '../.env') });

const wallpapersBucketName = process?.env?.WIDESCREEN_WALLPAPERS_BUCKET_NAME ?? ``;
const dynamoTableName = process?.env?.COLLECTOR_DYNAMODB ?? ``;
const blurhashQueueName = process?.env?.COLLECTOR_BLURHASH_QUEUE_NAME ?? ``;
const downloadWallpaperQueueName = process?.env?.COLLECTOR_DOWNLOAD_WALLPAPER_QUEUE_NAME ?? ``;

export class CollectorStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const everyTwoHoursCronJob = new events.Rule(this, 'everyTwoHoursCronJob', {
      schedule: events.Schedule.cron({ minute: `0`, hour: `0/2` }),
      ruleName: `everyTwoHoursCronJob`,
    });

    const blurhashQueue = new sqs.Queue(this, blurhashQueueName, {
      queueName: blurhashQueueName,
    });
    const downloadWallpaperQueue = new sqs.Queue(this, downloadWallpaperQueueName, {
      queueName: downloadWallpaperQueueName,
    });

    const table = new dynamo.Table(this, dynamoTableName, {
      tableName: dynamoTableName,
      partitionKey: { name: 'pk', type: dynamo.AttributeType.STRING },
      sortKey: { name: 'sk', type: dynamo.AttributeType.STRING },
      stream: dynamo.StreamViewType.NEW_IMAGE,
    });
    table.addGlobalSecondaryIndex({
      indexName: `${dynamoTableName}-media-type-index`,
      partitionKey: { name: `media_type`, type: dynamo.AttributeType.STRING },
    });

    const wallpapersBucket = new s3.Bucket(this, wallpapersBucketName, {
      blockPublicAccess: s3.BlockPublicAccess.BLOCK_ALL,
      bucketName: wallpapersBucketName,
    });

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

    const download_wallpaper_from_queue = new lambda.Function(this, `download_wallpaper_from_queue`, {
      handler: `main`,
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(path.resolve(__dirname, `./lambdas/download_wallpaper_from_queue/bootstrap.zip`)),
      functionName: `download_wallpaper_from_queue`,
    });
    download_wallpaper_from_queue.addEventSource(
      new SqsEventSource(downloadWallpaperQueue, {
        batchSize: 1,
      })
    );

    const get_wallpapers_from_source = new lambda.Function(this, `get_wallpapers_from_source`, {
      handler: `main`,
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(path.resolve(__dirname, `./lambdas/get_wallpapers_from_source/bootstrap.zip`)),
      functionName: `get_wallpapers_from_source`,
      // logRetention: logs.RetentionDays.ONE_DAY,
    });

    // Permissions
    blurhashQueue.grantSendMessages(get_wallpapers_from_source);
    blurhashQueue.grantConsumeMessages(attach_blurhash);

    downloadWallpaperQueue.grantSendMessages(attach_blurhash);
    downloadWallpaperQueue.grantConsumeMessages(download_wallpaper_from_queue);

    table.grantReadData(get_wallpapers_from_source);
    table.grantWriteData(download_wallpaper_from_queue);

    wallpapersBucket.grantWrite(download_wallpaper_from_queue);

    // Cron jobs
    everyTwoHoursCronJob.addTarget(new targets.LambdaFunction(get_wallpapers_from_source));
  }
}

const app = new cdk.App();
new CollectorStack(app, 'CollectorStack', {});
