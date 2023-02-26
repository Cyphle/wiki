# What is SQS ?

Pour Simple Queuing Service

# Notes

* Multiple producers can send messages to the same SQS queue
* Consumer poll messages, read it and delete the message (EC2, servers, lambda, ...)
* Multiple consumer can be linked to same queue but as message is deleted when a message is read, not for different services
* Fully managed service
* No limit on throughput and number of messages
* Default retention is 4 days and maximum is 14 days
* Low latency (<10ms)
* Max size of message 256kb
* SQS is at least once delivery
* Best effort of order or use FIFO type queue
* Batch size is max at 10 messages
* Metric available : queue length. Can be sent to cloudwatch
* Security
    * In flight with https
    * At rest with KMS
    * Client side if client perform encryption/decryption
    * Access controls: IAM policies
    * SQS Access Policies (similar to S3 policies)
        * Useful for cross account access
        * Useful for allowing other services (SNS, S3, ...) to write

# Access policy

* For EC2 instance to poll account, need an access policy attached to SQS Queue that allows the instance
* For S3 to publish event notification to SQS, need to attach policy to SQS to allow S3 to write (need that owner of bucket is the one to push)

# Message Visibility Timeout

* When a message is polled by something, it is invisible by others
* By default, visibility timeout is 30s, meaning poller has 30s to process the message and delete it
* API available ChangeMessageVisibility to increase timeout

# Dead letter queue

* If a message is failed to be processed, it returns to the queue
* Can set a threshhold of how many times a message can go back to queue
    * After MaximumReceives, threshold is exceeded, message goes to DLQ
* DLQ of a FIFO queue is FIFO and DLQ of standard queue is standard
* Retention of DLQ good to be set to 14 days
* Redrive to source: 
    * feature to help consume message in the DLQ to understand what is wrong
    * After fixing
    * Redrive messages from DLQ to queue

# Delay queue

* Delay a message up to 15min
* By default it is 0
* Can by overridenn with DelaySeconds parameter when sending message

# Long polling

* A consumer can wait before messages arrive => called long polling
* When a message arrives in queue, message goes to consumer
* Used to do less API calls to queue
* Between 1 to 20s
* Long polling is preferable to short polling
* Can be enable at queue level or at API level with ReceiveMessageWaitTimeSeconds

# Extended Client

* To send large messages, use SQS Extended Client
* Send big payload to S3 and send metadata message in SQS
    * Use extended client to synchronize

# Important API

* CreateQueue
* DeleteQueue
* PurgeQueue: delete all messages
* SendMessage
* ReceiveMessage
* DeleteMessage
* MaxNumberOfMessages: default 1, max 10
* ReceiveMessageWaitTimeSeconds: long polling
* ChangeMessageVisibility: message timeaout
* Batch
    * SendMessage
    * DeleteMessage
    * ChangeMessageVisibility

# FIFO Queue

* Ordered queue
* Limited throughput 300msg/s without batching and 3000 with
* Exactly once send capability (remove duplicates)
* Name has to finish with .fifo
* Deduplication
    * Interval is 5min
    * Content-based on SHA-256
    * Provide message deduplication ID
* Message grouping with MessageGroupID
    * Only one consumer for group
    * For order of a subset of message, specify different MessageGroupID
    * Messages with same group ID are ordered in the group
    * Ordering across groups is not guaranteed