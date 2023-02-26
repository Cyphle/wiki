# What is DynamoDB ?

* Serverless column database. Do not manage server.
* Column database like Cassandra, meaning dynamic columns (columns do not have to exist for all rows)

# Notes
* Scale horizontally
* Highly available, replication accross multiple AZ
* Distributed database
* Millions of requests per resonds
* Trillions of rows
* 100s of TB of storage
* Fast and consistent performance (low latency on retrieval)
* Integrated with IAM
* Event driven with DynamoDB stream
* Low cost
* Auto scalling
* Standard and Infrequent access table class
* Each table has a primary key
    * Option 1: Partition key (hash)
    * Option 2: parition key + sort key (hash + range) -> combination of both must be unique (composite key)
* Each item (a row) has attributes (= column)
* Item is max 400Kb
* Data types
    * Scalar: string, number, binary, boolean, null
    * Document types: List, map
    * Set types: string set, number set, binary set
* Read/Write capacity:
    * Ondemand
    * Provisioned (has free tier)
* Can only query on primary key (or composite primary key) or on indexes

# DynamoDB WCU & RCU
* Read/Write capacity modes
    * Control how to anage table capacity
    * Provisioned mode (default)
        * Specify number of reads/writes per second
        * Pay for provisioned
    * On-Demande mode
        * Reads/Writes scale up/down with workload
        * Pay for use but more expensive
    * Can switch between different modes once every 24hours
* Provisioned mode
    * Read Capacity Units (RCU): throughput
    * Write Capacity Units (WCU): throughput
    * Option to setup auto scaling
    * Throughput can be exceeded temporarily using "Burst capacity"
    * If burst capacity has been consumer, get a ProvisionedThroughputExceededException
    * Advised to do an exponential backoff retry then
* When on-demand
    * No capacity planning
    * Unilimited RCU and WCU
    * Around 2.5x more expensive
    * Use when unknown or unpredictable traffic
* Write
    * WCU
        * One WCU represents one write per second for an item up to 1kb
        * If items are larger than 1kb, more WCU are consumed (tailles non entière sont arrondis au-dessus)
        * Example: 10 items per second of 2kb = 20 WCU needed
* Read
    * Read modes: strongly consistent ready & eventually consistent read
        * Eventually consistent read (default mode)
            * In this mode, read are in eventual consistency
        * Strongly consistent read
            * In this mode, read are synchronous
            * Set "ConsistentRead" parameter to true in API calls
            * Consumes twice the RCU
    * RCU
        * 1 RCU = one strongly consistent read per second or two eventually consistent reads per second for items up to 4kb
        * If items are larger, then consume more RCU
* Partitions internal
    * Each table has partitions, each partition is on different server
    * Decided on hash through hashing algorithm (AWS internal)
    * Number of partition
        * number by capacity = (RCU total/3000) + (WCU total/1000)
        * number by size = Total size table/10Gb
        * number of partition = ceil(max(by capacity, by size))
    * Throttling
        * If exceed provisioned RCU or WCU, reasons might be
            * Hot keys: one partition key is being read too many times
            * Hot partitions
            * Very large items
        * Solution
            * Exponential backoff when exception is encountered (already in SDK)
            * Distribute parition keys as much as possible
            * If RCU issue, can use DynamoDB Accelerator (DAX)

# API
* PutItem
    * Create a new item or fully replace item with same primary key
    * Consume WCU
* UpdateItem
    * Edit existing item attributes or add new if does not exist
    * Can be used to implement atomic counters (number attribute that's unconditionally incremented)
* Conditional writes
    * Accept write/update/delete only if conditions are met otherwise error
    * Help with concurrent access
    * No performance impact
* GetItem
    * Read based on primary key
    * Use ProjectionExpression: can be specified to retrieve only certain attributes
* Query
    * Returns item based on
        * KeyConditionExpression
            * Partition key value (= operator)
            * Sort key (=, <, <-, ...)
        * FilterExpression
            * Use only with non key attribute
    * Returns list of imits
        * Number returned specified in Limit
        * Up to 1Mb of data
    * Can paginate
    * Can query table, local secondary index, global secondary index
* Scan
    * The read entire table and then filter out data (inefficient)
    * Return up to 1Mb, for more use pagination
    * Consumes lots of RCU as entire table is read
    * Use Limit statment or reduce size to avoir performance
    * Can use parallel scan for performance
    * Can use ProjectionExpression & FilterExpression
* DeleteItem
    * Individual
    * Conditional delete
* DeleteTable
    * Delete whole table
    * Much quicker than calling DeleteItem on all items
* Can perform batch operations
    * Part of batch can fail, retry for failed items
* BatchWriteItem
    * Up to 25 PuItem and/or DeleteItem
    * Up to 16Mb of data written, up to 400Kb per item
    * Can't update items
* BatchGetItem
    * Return from multiple tables
    * Up to 100 items
    * Up to 16Mb of data
    * Items are retrieved in parallel

# Indexes
* Local secondary Index (LSI)
    * Alternative sort key (additional sort key). Not alternate of attribute to make them primary key, use global index
    * Index key with String, number or binary
    * Up to 5 LSI per table
    * Must be defined at table creation
    * Attribute projections: can contain some or all attributes (KEYS-ONLY, INCLUDE, ALL)
* Global secondary index (GSI)
    * Alternative primary key (hash or hash + range)
    * Speed up queries on non key attributes
    * Index key with String, number or binary
    * Attribute projections
    * Must provision RCU & WCU for index
    * It is like a new table
    * Can be added or modified after table creation
* Throttling
    * GSI
        * If the writes are throttled on GSI, then main table will be throttled
        * Event if WCU on main tables are fine
        * Assign WCU capacity carefully
    * LSI
        * Uses WCU and RCU of main table
        * No special consideration for throttling

# PartiQL
* Use SQL-like syntax to manipulate tables
* Supports (some but not all)
    * Insert
    * Update
    * Select
    * Delete
* Support batch operations

# Optimistic locking
* Feature called "Conditional Writes"
* Strategy to ensure an item has not changed before update/delete -> optimistic locking
* Each item has an attribute that act as a version number
* Example: update only if version = 1

# DAX: DynamoDB Accelerator
* Fully managed, highly available, seamless in memory cache
* Microseconds latency
* Compatible with existing, no modification required
* Solves hot key problem (too many reads)
* 5min TTL by default
* Work with cache clusters
    * Up to 10 nodes in cluster
* Multi AZ (3 nodes minimum recommended for prod)
* Secure: encryption with KMS, CloudTrail, etc
* DAX vs ElastiCache
    * Can be used as combination
    * With DAX
        * Cache for individual items, query and scan
    * With ElastiCache
        * For calculated results like aggregations, sums

# DynamoDB Stream
* Ordered stream of item level modifications in a table (create/update/delete)
* Stream records can be
    * Sent to Kinesis Data Stream
    * Read by AWS Lambda
    * Read by Kinesis Client library (KCL)
* Data retention for up to 24 hours
* Use cases:
    * react in real time to modifications
    * analytics
    * insert into derivative tables
    * insert into elastic search
    * implement cross region replication
* Ability to choose the information in stream
    * KEYS_ONLY: only keys of modified attributes
    * NEW_IMAGE: item in new form
    * OLD_IMAGE: item in old form
    * NEW_AND_OLD_IMAGES: both new and old form
* Streams are made of shards like Kinesis Data Stream
* Do not provision shards, automated by AWS
* It is not retroactive, stream populated only after activation
* For lambda, need to use Event Source Mapping to be invoked synchronously

# TTL: Time to live
* Automatically delete items after an expiry timestamp (in Unix Epoch timestamp)
* Does not consume WCU
* Effective delete is within 48 hours (can be configured) of expiration (first expire and then delete)
* Expired items still appear in queries
* Expired items are deleted from LSI and GSI
* A delete operations for each expired item enters the stream
* Use cases
    * Reduce stored data
    * Regulatory
* Set TTL on the column name which has to be a timestamp

# CLI
* --projection-expression: to retrieve subset of attributes
* --filter-expression: filter items
* Pagination options
    * --page-size: default 1000, retrives full list of items but with a larger number of API calls instead of one API call
    * --max-items: max number of items to show and return NextToken
    * --starting-token: specify last NextToken to retrieve next set of items

# Transactions
* Coordinated, all or nothing operations (add/update/delete) to multiple items across multiple tables
* Gives ACID properties: Atomicity, Consistency, Isolation and Durability
* Read modes
    * Eventual consistency
    * Strong consistency
    * Transactional consistency
* Write modes
    * Standard
    * Transactional
* Transaction use 2x WCU & RCU
    * Prepare transaction and commit
* Two operations
    * TransactGetItems
    * TransactWriteItems

# Session state cache
* Can use DynamoDB to store appication session state
* vs ElastiCache
    * ElastiCache is in-memory
    * DynamoDB is serverless
    * Both key/value pairs
* vs EFS
    * EFS must be atteched as network drive to EC2
* vs EBS & instance store
    * EBS & Instance only local caching of EC2
* vs S3
    * S3 has higher latency, not meant for small objects

# Partition strategy (Write sharding)
* Example: 
    * candidate A and candidate B to vote
    * if candidate_id has partition key => two partitions
    * Will generate hot partition
    * Strategy is to add suffix or prefix to partition key, example candidate_A_11
* Two methods to create suffix
    * Random suffix
    * Calculated suffix

## Write types
* Concurrent writes
    * If multiple write to same item, both succeed but one will be lost
    * To solve, use conditional write
* Conditional write (= optimistic locking)
    * Add a condition like version checking
* Atomic write
    * One add 1 and one add 2 => increment by 3 (both succeed)
* Batch write
    * Update many items

## Large objects pattern
* Do not store large objects because DynamoDB is 400kb max
* Upload in S3 the object and store its reference and meta data

## Operations
* Table cleanup
    * Option 1 : scan + deletion
        * Very slow, consumes RCU and WCU
    * Option 2 : drop table + recreate table
* Copy table
    * Option 1 : use AWS Data pipeline
    * Option 2 : back up and restore
    * Option 3 : scan and batch put or write

## Security and other features
* VPC Endpoint available to access without internet
* Access fully controled by IAM
* Encryption with KMS (at rest) and SSL/TLS (in transit)
* Backup and restore
    * Point in time (PITR) like RDS
    * No performance impact
* Global tables concept available
    * Multi region, multi active, fully replicated, high performance
* DynamoDB local
    * Can have a local DynamoDB for dev purposes
* To migrate to and from DynamoDB, AWS Data Migration Service
* Fined grained access
    * Do not want to provide IAM access for each user
    * Use other IDP like Google, Cognito, etc to get a temporary AWS credentials associated to IAM role
    * Can assign IAM role to users with a condition to limit their API access to DynamoDB. Conditions examples
        * LeadingKeys: limit row level access for users on the primary key
        * Attribute: limit specific attributes
