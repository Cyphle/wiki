# What is Kinesis ?

* Make it easy to collect, process and analyze streaming data in real-time
* For logs, metrics, clickstream, iot telemetry data
* Products
    * Kinesis Data Streams: capture, process and store data streams
    * Kinesis Data Firehose: load data streams into AWS data stores (in AWS and outside AWS)
    * Kinesis Data Analytics: analyze data stream with SQL or Apache Flink
    * Kinesis Video Streams: capture, process and store video streams

# Kinesis Data Streams

* To stream big data in system
* Works with shards (must be provisioned ahead of time): data split into shards
    * It is stream capacity and consumption capacity
* Producers
    * Types
        * Application
        * Client
        * SDK, KPL (Kinesis Producer Library)
        * Kinesis Agent
    * Produced records with partition key and data blob
    * Throughput: 1MB/s or 1000msg/s per shard
* Consumers
    * Types
        * Apps (KCL (Kinesis Consumer Library), SDK)
        * Lambda
        * Kinesis Data Firehose
        * Kinesis Data Analytics
    * Receive partition key, sequence number, data blob
    * Throughput: 2MB/s per shard for all consumers or 2MB/s per shard per consumer with enhanced version
* Retention is between 1 day to 365 days
* Ability to replay data
* Once data is inserted, can't be deleted
* Data that go to same shard are ordered
* Capacity mode
    * Provisioned mode: 
        * choose number of shards
        * pay per shard per hour
    * On-demand mode: 
        * manage capacity. Default 4MB/s or 4000 records/s
        * Scale based on observed throughput peak during last 30 days
        * Pay per stream per hour & data in/out per GB
* Shard iterator types
    * TRIM_HORIZON: read from the beginning then get another iterator
    * SharedIterator: iterator récupéré après la lecture et permet d'avancer dans le flux quand on lit

## Security

* Control access with IAM policies
* Encryption with HTTPS and KMS
* Can implement client side
* VPC endpoints are available to access within a VPC
* Monitor API calls with CloudTrail

## Producers
* Put data records into data streams which have
    * Sequence number (unique per partition-key with shard)
    * Partition key (ex: device id)
    * Data blob (up to 1Mb)
* Libraries
    * SDK: simple producer
    * Kinesis Producer Library (KPL): C++, java, ..., add compression, retries
    * Kinesis agent to monitor log files (standalone application)
* Can use batching when PutRecords
* All data that share same partition key go to same shard
* Assure that partition key are balanced otherwise error of hot partition with shard that has too many data
* ProvisionedThroughputExceeded
    * When a producer put too many data
    * To avoir error,
        * highly distributed partition keys
        * Retries with exponential backoff
        * Increase shards (scaling/shard splitting)

## Consumers
* Get records from the streams
* Can be
    * lambda
    * Kinesis data analytics
    * Kinesis data firehose
    * Custom
    * Kinesis Client Library (KCL)
* Consumption modes
    * Shared (classic) fan-out consumer
        * 2MB/s per shard across all consumers
        * With GetRecords()
        * It is pull method
            * Low number of consuming apps
            * Max 5 GetRecords call/s
            * Latency 200ms
            * Minimize cost
            * Consumers poll data from Kinesis
            * Returns up to 10Mb then throttle for 5s or up to 10000 records
    * Enhanced fan-out consumer
        * 2MB/s per consumer per shard
        * With SubscribeToShard()
        * It is push method
            * Multiple consumers for same stream
            * Latency 70ms
            * Higher cost
            * Kinesis push to consumers with SubscribeToShard
            * Limit of 5 consumer apps but can increase by demanding with AWS ticket
* Lambda consumer
    * Supports classic and enhanced fan-out consumer mode
    * Read records in batches
    * Configure batch size and window
    * Lambda retries when error or data expires
    * Up to 10 batches per shard at same time
* Kinesis Client Library
    * A java library that helps read records with distributed applications sharing read workload
    * Each shard is to be read by only one KCL instance (so one app per shard max but one app can read multiple shards)
    * Progress is checkpointed into DynamoDB (needs IAM access)
    * Track other workers and share work using DynamoDB
    * Run on EC2, beanstalk, on premise
    * Records are read in order at shard level
    * Versions
        * 1.x: support shared consumer
        * 2.x: support shared consumer and enhanced fan-out

## Operations
* Shard splitting
    * Used to increase stream capacity
    * Used when needing to divide a hot shard
    * Old shard will be closed and data are expired then deleted
    * No automatic scaling
    * Cannot split in more than 2 shards at a time
* Shard merging
    * Want to save cost
    * Group shards
    * Can't merge more than two shards at a time


# Kinesis Data Firehose
* Can take data from producers
    * Application
    * Client
    * SDK, KPL
    * Kinesis Agent
    * Kinesis Data Streams
    * CloudWatch
    * AWS IoT
* Can choose to transform data with lambda
* Write in batch to destination
* Destinations
    * S3
    * Redshift (Copy through S3)
    * AWS OpenSearch
    * Datadog
    * Splunk
    * NewRelic
    * MongoDB
    * Custom destinations (HTTP endpoint)
* Can send to S3 as backup or only failed
* Fully managed service: automatic scaling, serverless
* Pay for data going through Firehose
* Near real time: 60s latency for non full batches or wait for 1MB at a time
* Support data formats, conversions, transformation, compression, ...
* Buffer size: size of batch
* Buffer interval: how long to wait for a batch to attain buffer size, afterwards send even if size is not

## Data Stream vs Data Firehose
* Data Stream
    * Streaming service for ingest at scale
    * Storage 1 to 365 days
    * Support replay
* Data Firehose
    * Load stream data to send to S3, Redshift, ...
    * Near real time
    * Automatic scaling
    * No data storage (no replay)


# Kinesis Data Analysis
* Can read from Kinesis Data Stream or Data Firehose
* Can add reference data from S3 (enrich data in real time)
* Use SQL statements to query data (query, filter, etc)
* Destinations are sinks. Can send to
    * Kinesis Data Streams
    * Kinesis Data Firehose
* It is real time analytics
* Fully manage (automatic scaling)
* Pay for consumption rate
* Use cases
    * Time series analytics
    * Real-time dashboards
    * Real-time metrics
* Data Analytics for Apache Flink
    * Use flink to process and analyse streaming data
    * Can read from Data Streams or Amazon MSK
    * Run Apache Flink application on managed cluster on AWS to enhance Kinesis Data Analytics for SQL
    * Has backups

# SQS vs SNS vs Kinesis
* SQS
    * Consumer pull data
    * When data is processed, data is deleted
    * As many workers as needed on same queue
    * Manage service
    * Order with FIFO
    * Individual message delay
* SNS
    * Push data (pubsub)
    * Up to 12 500 000 subscribers
    * No persistence (if message not read, is lost)
    * Up to 100 000 topics
    * Managed service
    * Integrates with SQS
    * Order with FIFO
* Kinesis
    * Pull data or push data (enhanced fan-out)
    * Can replay data with Data streams
    * For real time big data, analytics and ETL
    * Order in same shard
    * Data expires after X days
    * Provision capacity mode and ondemand capacity mode