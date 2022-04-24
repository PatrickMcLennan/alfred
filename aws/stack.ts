import * as api from '@aws-cdk/aws-apigateway';
import * as cdk from '@aws-cdk/core';
import * as cognito from '@aws-cdk/aws-cognito';
import * as dynamo from '@aws-cdk/aws-dynamodb';
import * as events from '@aws-cdk/aws-events';
import * as lambda from '@aws-cdk/aws-lambda';
import * as path from 'path';
import * as s3 from '@aws-cdk/aws-s3';
import * as sqs from '@aws-cdk/aws-sqs';
import * as targets from '@aws-cdk/aws-events-targets';
import { SqsEventSource } from '@aws-cdk/aws-lambda-event-sources';

import { config } from 'dotenv';

config({ path: path.resolve(__dirname, '../.env') });

const wallpapersBucketName = process?.env?.WIDESCREEN_WALLPAPERS_BUCKET_NAME ?? ``;
const cognitoPoolName = process?.env?.COLLECTOR_USER_POOL_NAME ?? ``;
const dynamoTableName = process?.env?.COLLECTOR_DYNAMODB ?? ``;
const blurhashQueueName = process?.env?.COLLECTOR_BLURHASH_QUEUE_NAME ?? ``;
const downloadWallpaperQueueName = process?.env?.COLLECTOR_DOWNLOAD_WALLPAPER_QUEUE_NAME ?? ``;

const standardCognitoAttributes = {
  givenName: true,
  familyName: true,
  email: true,
  emailVerified: true,
  address: true,
  birthdate: true,
  gender: true,
  locale: true,
  middleName: true,
  fullname: true,
  nickname: true,
  phoneNumber: true,
  phoneNumberVerified: true,
  profilePicture: true,
  preferredUsername: true,
  profilePage: true,
  timezone: true,
  lastUpdateTime: true,
  website: true,
};
export class Alfred extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    /**
     * Cognito Pools
     */
    const userPool = new cognito.UserPool(this, cognitoPoolName, {
      userPoolName: cognitoPoolName,
      selfSignUpEnabled: false,
      signInAliases: {
        email: true,
      },
      autoVerify: {
        email: true,
      },
      standardAttributes: {
        familyName: {
          required: true,
          mutable: true,
        },
        givenName: {
          required: true,
          mutable: true,
        },
      },
      customAttributes: {
        isAdmin: new cognito.BooleanAttribute({ mutable: true }),
      },
      passwordPolicy: {
        minLength: 12,
        requireLowercase: true,
        requireDigits: true,
        requireUppercase: true,
        requireSymbols: true,
      },
      accountRecovery: cognito.AccountRecovery.EMAIL_ONLY,
      removalPolicy: cdk.RemovalPolicy.RETAIN,
    });

    const userPoolClient = new cognito.UserPoolClient(this, `alfred-cognito-provider`, {
      accessTokenValidity: cdk.Duration.minutes(5),
      userPoolClientName: `alfred-cognito-provider`,
      userPool,
      authFlows: {
        userPassword: true,
      },
      supportedIdentityProviders: [cognito.UserPoolClientIdentityProvider.COGNITO],
      readAttributes: new cognito.ClientAttributes()
        .withStandardAttributes(standardCognitoAttributes)
        .withCustomAttributes(...['isAdmin']),
      refreshTokenValidity: cdk.Duration.days(5),
      writeAttributes: new cognito.ClientAttributes()
        .withStandardAttributes({
          ...standardCognitoAttributes,
          emailVerified: false,
          phoneNumberVerified: false,
        })
        .withCustomAttributes(...[]),
    });

    /**
     * Cron schedules
     */
    const everyTwoHoursCronJob = new events.Rule(this, 'everyTwoHoursCronJob', {
      schedule: events.Schedule.cron({ minute: `0`, hour: `0/2` }),
      ruleName: `everyTwoHoursCronJob`,
    });

    /**
     * SQS Queues
     */
    const blurhashQueue = new sqs.Queue(this, blurhashQueueName, {
      queueName: blurhashQueueName,
    });
    const downloadWallpaperQueue = new sqs.Queue(this, downloadWallpaperQueueName, {
      queueName: downloadWallpaperQueueName,
    });

    /**
     * API Gateway
     */

    const integrationResponse = new api.Integration({
      type: api.IntegrationType.HTTP,
      options: {
        integrationResponses: [
          {
            statusCode: 'statusCode',
            responseParameters: {
              'method.response.header.Set-Cookie': '$context.authorizer.Set-Cookie',
            },
          },
        ],
      },
    });

    const restApi = new api.RestApi(this, `alfred-api`, {
      restApiName: `alfred-api`,
      defaultIntegration: integrationResponse,
      description: `Rest API for Alfred`,
    });

    const restApiRoot = restApi.root.addResource('api');
    const authApi = restApiRoot.addResource('auth');
    const wallpapersApi = restApiRoot.addResource('wallpapers');

    const loginRoute = authApi.addResource('login');
    const logoutRoute = authApi.addResource('logout');

    const authorizer_lambda = new lambda.Function(this, `alfred-authorizer-lambda`, {
      handler: `main`,
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(path.resolve(__dirname, `./lambdas/authorizer/bootstrap.zip`)),
      functionName: `authorizer`,
    });
    const authorizer = new api.RequestAuthorizer(this, `alfred-request-authorizer`, {
      handler: authorizer_lambda,
      identitySources: [api.IdentitySource.header('Cookie')],
    });

    /**
     * Dynamo Tables
     */
    const table = new dynamo.Table(this, dynamoTableName, {
      tableName: dynamoTableName,
      partitionKey: { name: 'pk', type: dynamo.AttributeType.STRING },
      sortKey: { name: 'sk', type: dynamo.AttributeType.STRING },
      // stream: dynamo.StreamViewType.NEW_IMAGE,
    });

    /**
     * S3 buckets
     */
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

    /**
     * Lambdas
     */
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

    const login = new lambda.Function(this, `login`, {
      handler: `main`,
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(path.resolve(__dirname, `./lambdas/login/bootstrap.zip`)),
      functionName: `login`,
      // logRetention: logs.RetentionDays.ONE_DAY,
    });

    const logout = new lambda.Function(this, `logout`, {
      handler: `main`,
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(path.resolve(__dirname, `./lambdas/logout/bootstrap.zip`)),
      functionName: `logout`,
      // logRetention: logs.RetentionDays.ONE_DAY,
    });

    const search_wallpapers = new lambda.Function(this, `search_wallpapers`, {
      handler: `main`,
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(path.resolve(__dirname, `./lambdas/search_wallpapers/bootstrap.zip`)),
      functionName: `search_wallpapers`,
      // logRetention: logs.RetentionDays.ONE_DAY,
    });

    /**
     * Permissions
     */
    blurhashQueue.grantSendMessages(get_wallpapers_from_source);
    blurhashQueue.grantConsumeMessages(attach_blurhash);

    downloadWallpaperQueue.grantSendMessages(attach_blurhash);
    downloadWallpaperQueue.grantConsumeMessages(download_wallpaper_from_queue);

    table.grantReadData(get_wallpapers_from_source);
    table.grantReadData(search_wallpapers);
    table.grantWriteData(download_wallpaper_from_queue);

    wallpapersBucket.grantWrite(download_wallpaper_from_queue);

    /**
     * API Routes
     */
    loginRoute.addMethod(`POST`, new api.LambdaIntegration(login));
    logoutRoute.addMethod(`POST`, new api.LambdaIntegration(logout));
    wallpapersApi.addMethod(`POST`, new api.LambdaIntegration(search_wallpapers), {
      authorizer,
    });

    /**
     * Cron jobs
     */
    everyTwoHoursCronJob.addTarget(new targets.LambdaFunction(get_wallpapers_from_source));

    new cdk.CfnOutput(this, 'userPoolId', {
      value: userPool.userPoolId,
    });
    new cdk.CfnOutput(this, 'userPoolClientId', {
      value: userPoolClient.userPoolClientId,
    });
    new cdk.CfnOutput(this, 'apigatewayId', {
      value: restApi.restApiId,
    });
  }
}

const app = new cdk.App();
new Alfred(app, 'Alfred', {});
