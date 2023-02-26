# What is SNS ?

Pour Simple Notification Service

# Notes

* One SNS topic have multiple subscribers
* Pub-Sub pattern
* Event producer sends to one SNS topic
* As many event receiver and all will get the messages (can filter messages)
* Up to 12 500 000 subscriptions per topics
* Up to 100 000 topics
* Subcribers
    * Emails
    * SMS
    * HTTP
    * SQS
    * Lambda
    * Kinesis Firehose
* Producers
    * CloudWatch alarm
    * Budget
    * ASG
    * S3
    * ...
* To use: 
    * Topic Public with SDK
        * Create topic
        * Create subscription
        * Public to the topic
    * Direct publish (mobile SDK)
        * Create a platform application
        * Create a platform endpoint
        * Public to platform endpoint
        * Subscribers: Google GCM, Apple APNS, Amazon ADM, ...

# Security

* Encryption
    * Inflight HTTPS
    * At rest with KMS
    * Client side
* IAM policies to regulate access to API
* SNS Access policies (similar to S3 policies)

# Fan out pattern (SQS + SNS)

* Push once in SNS, receive in many SQS that are subscribers
* It is to resolve the principle of SQS that consumers delete messages after processing
* SNS has to have right to write to SQS
* Cross region delivery: SNS can send to SQS in other regions
* Example use cases
    * S3 Events to multiple queue
        * For same combination of event type (ex object create) and prefix (ex images/) can only have one S3 event rule
        * If want to send same S3 event to many SQS, use fan-out pattern
    * SNS to S3 through Kinesis Data Firehose
        * SNS send to Kinesis and S3 receive it

# FIFO

* SNS can be FIFO
* Same features as SQS: group id, deduplication
* Only SQS FIFO as subscribers

# Message filtering

* JSON policy to filter messages to subscribers