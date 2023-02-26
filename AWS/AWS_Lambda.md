# What is AWS Lambda ?

* Serverless execution of small code (functions)

# Notes
* Serverless: do not manage server, just deploy code
* Serverless in AWS:
    * Lambda
    * DynamoDB
    * Cognito
    * API Gateway
    * S3
    * SNS & SQS
    * Kinesis Data Firehose
    * Aurora serverless
    * Step functions
    * Fargate
* Has to have short executions
* Run on demand
* Automatic scaling
* Pricing
    * Pay per request
    * Compute time (how long it runs)
    * Free tier: 1 000 000 lambda requests and 400 000 Gbs of compute time
* Monitoring with CloudWatch
* Up to 10Gb of RAM per function
* Has to run lambda container image
    * Image must implement lambda runtime API
    * ECS/Fargate is prefered to run other Docker images
* To use need to define a handler in code
```
def lambda_handler(event, context):
    ...
```

# Integration
* Integration with for example
    * API Gateway: create REST api to invoke lambda
    * Kinesis
    * DynamoDB: if something happens, lambda is triggered
    * S3
    * CloudFront: lambda at edge
    * CloudWatch Events/EventBridge
    * CloudWatch logs
    * SNS
    * SQS
    * Cognito

# Languages
* NodeJS
* Python
* Java 8+
* C#
* Golang
* C#/Powershell
* Ruby
* Custom runtime API (like Rust)

# Load Balancing
* With ALB
    * To expose lambda as an HTTP endpoint, can use ALB or API Gateway
    * Lambda must be registered in a target group
    * To invoke, ALB transform HTTP to JSON
    * Multi-header values can be set
        * It is a setting on ALB
        * Exemple: http://example.com?name=foo&name=bar will send { "name": ["foo", "bar"]}

# Synchronous & Asynchronous
* Synchronous invocations
    * When using CLI, SDK, API Gateway, Application load balancer, cloudfront, ...
    * Error must by handled by cliend side (retries, exponential backoff, etc)
* Asynchronous invocation
    * S3, SNS, CloudWatch Events/EventBridge, AWS CodeCommit, CodePipeline,... invoke asynchronously
    * Events are placed in Event Queue and lambda reads queue
    * If error, retries 3 times 1min after and 2min after
    * Can define DLQ (SNS or SQS), lambda sends events to SNS ou SQS

# At Edge
* Deploy lambda with CloudFront CDN
* Can modify response and request
    * Viewer request: request sent to cloud front
    * Origin request: cloud front to origin
    * Origin response: origin to cloud front
    * Viewer response: cloud front to user
* Use cases
    * Website security and privacy
    * Dynamic web application
    * SEO
    * Intelligent route accross origins and data centers
    * Bot mitigation
    * Real time image transformation
    * A/B testing
    * User authentication and authorization
    * User priorization
    * User tracking and analytics

# CloudWatch Events/EventBridge
* Lambda needs role to write to CloudWatch logs
* To integrate, examples
    * A cron (ex: every 1hour perform a task)
    * CodePipeline set EventBridge Rule to launch a task

# S3 Event notification
* S3 can send events to 
    * SNS and fan-out pattern
    * SQS to invoke lambda
    * Directly lambda (asynchronous)

# Lambda Event Source Mapping
* Apply to
    * Kinesis Data Streams
    * SQS & SQS FIFO
    * DynamoDB streams
* Records need to be polled from the source (lambda poll from the service)
* Synchronous invoke
* Event source mapping poll from service and send to lambda
* With streams (Kinesis & DynamoDB stream)
    * Iterator for each shard and processed in order for each shard
    * Records processed are not removed from source
    * When low traffic can use batch window to accumulate records before processing
    * To accelerate can process multiple batches in parallel (10 batch per shard and ordered at partition key level)
    * If error, entire batch is reprocessed
        * Can discard error record and send to destination
        * Restrict number of retries
        * Split the batch
* With queues (SQS & SQS FIFO)
    * Event source mapping poll event and invoke lambda
    * Batch size 1 to 10
    * Queue visibility time to 6x timeout of lambda is recommended
    * Set DLQ on SQS and not on lambda or use lambda destination
    * Number of lambdas == number of active message group
    * Occasionally can receive event twice even without error
    * Lambda has to delete treated events
* Lambda event mapper scaling
    * With stream: one lambda invokatin per stream shard and up to 10 batches
    * With standard queues: up to 60 more instances and up to 1000 batches
    * With FIFO queue: scale == number of message groups

# Lambda destination
* Send the result of asynchronous invokation (success and fail) to
    * SQS
    * SNS
    * Lambda
    * EventBridge
* Event source mapping send when discarded event batches to
    * SQS
    * SNS
    * Avoid blocking treatment
* Prefer destination instead of DLQ (allow more destination)

# Permissions
* Attach IAM role to lambda to access resources and services
    * AWSLambda... policies can be used
* When using event source mapping must use an execution role to read event data
* Best practice : one lambda execution role per function
* Lambda resources based policies are used to give permission to other services to invoke lambdas
    * Similar to S3 policies
    * Attach policy to lambda to allow the S3
* An IAM principal can access lambda if
    * If the IAM policy attached to the principal authorizes
    * Or if resource-based policy authorize (ex: service access)
    * Exemple: when service like S3 calls lambda, resource based policy gives it access
* Resume:
    * Role to allow lambda to access resources
    * Resources based policies to allow other resource to invoke lambda

# Environment variables
* Can be encrypted with KMS

# Logs & Monitoring
* Logs are stored in CloudWatch logs. Make sure lambda has execution role with IAM policy to write in CloudWatch
* Logs
    * Invocations
    * Durations
    * Concurrent executions
    * Error count
    * Success rates
    * Throttles
    * Async delivery failure
    * Iterator Age (Kinesis & DynamoDB streams)
* Can use X-ray to trace
    * Use X-Ray SDK
    * Add role to use X-Ray
    * Set environment variables to communicate with X-Ray

# Lambda in VPC
* By default are launched outside VPC (in AWS owned VPC), so cannot access resources in VPC (RDS, ElastiCache, ELB, ...)
* Can access internet and other services outside VPC like DynamoDB
* But can configure the VPC and subnet IDS and Security group
    * Lambda will create an ENI in subnets
    * Need AWSLambdaVPCAccessExecutionRole
* Once connected to VPC, lambda does not have access to internet
    * Deploying lambda in public subnet does not give internet access
    * To give internet, deploy in private subnet and give internet with Net Gateway/Instance in public subnet which talks to internet gateway in public subnet (use route tables)
    * Then to access DynamoDB can access using internet or privately with VPC endpoint

# Performance
* RAM: 
    * From 128Mb to 10Gb in 1mb increments 
    * The more RAM, the more vCPU. At 1792Mb get 1 vCPU, after that need to use multi threading to benefit it
* Cannot set CPU, linked to RAM configuration
* By default, 3s timeout and maximum 15min (has to run in 3s by default)
* Execution context
    * Temporary runtime environment that initializes external dependencies
    * For database connections, HTTP clients, SDK clients
    * Context is maintained for some time in anticipation of another invocation
    * Includes /tmp directory (10Gb of disk space)
        * Is a transient cache that can be used for multiple invocations
        * For permanent persistence use for example S3
* Context use example
```
import os

// Defined outside handler so in the context (can be reused and improve performance)
DB_URL = os.getenv("DB_URL")
db_client = db.connect(DB_URL)

def get_user_handler(event, context):
    user = db_client.get(user_id = event("user_id"))
    return user
```

# Concurrency
* Up to 1000 concurrent executions. Open a request ticket to have more
* Can be limit with "reserved concurrency". Set a function level for example 50 concurrent executions
* Each invocation over the limit trigger a "Throttle"
    * If synchronous invocation, return ThrottleError (429)
    * If asynchronous invocation, return automatically and then go to DLQ
* If no limit is set, issues can:
    * If a lambda is called by multiple ways, if a way takes all slots of execution, other are blocked
    * When asynchronous, events can be sent to DLQ and are retried for up to 6hours. Retries are exponential from 1s after first attempt to 5min
* Cold start:
    * New instance, code is loaded and code outside the handler run (initialization)
    * If the init is large process can take some time
    * First request has higher latency than the rest
* To solve cold start, use provisioned concurrency (it is a warm pool of concurrency)
    * Concurrency is allocated before the function is invoked (in advance)
    * So the cold start never happens and all invocations have low latency
    * Can use application auto scaling to manage concurrency (schedule or target utilization)
* Can reserve concurrency to set concurrency in advance

# Dependencies
* Need to install the packages with the code and zip it together and upload zip to lambda if less than 50Mb else use S3 first
* Native libraries work
* AWS SDK comes by default

# Lambda and CloudFormation
* To create with CloudFormation
    * Inline: put code in the definition of template
    * Through S3: refer the zip (bucket, full path, version)
    * Through S3 to deploy in multiple accounts: add bucket policies

# Lambda layers
* Allow to
    * Create custom runtimes (ex: C++ and Rust)
    * Externalize dependencies to reuse
* Similar to Docker layer   

# Container images
* Can deploy lambda from container images up to 10Gb from ECR
* Base image has to contain Lambda Runtime API
* Some base images: NodeJS, Python, Java, .NET, Go, Ruby
* Can create own image
* Can test image locally using Lambda Runtime Interface Emulator

# Versions and Alias
* Lambda supports versions and aliases of version
* Aliases are pointers to versions
* Aliases can be used to create like 'dev' and 'prod' aliases and move the pointer to stable functions without changing definition of version used
* Aliases can enable canary deployment by assigning wieghts to lambda functions
* Alias cannot reference alias

# Lambda & CodeDeploy
* CodeDeploy can automate deployment
* Can automate traffic shift for lambda aliases: progressive deployment
* Feature is integrated within SAM framework
* Progressive deployment strategies
    * Linear: grow traffic every N minutes
    * Canary: try X percent then 100%
    * AllAtOnce
* Rollbackups
    * Can create Pre & Post traffic hooks to check health

# Limits
* Limits are per region
* Execution limits:
    * Memory: 128Mb to 10Gb with 1Mb increments
    * More memory -> more CPU
    * Maxmimum execution time: 900s (15min)
    * Env variable max 4Kb max
    * Disak capacity /tmp: 512Mb to 10Gb
    * 1000 concurrent executions
* Deployment limits:
    * Compressed: 50Mb
    * Uncompressed: 250Mb (code + dependencies)
    * Use /tmp to load other files at startup
    * Env var 4Kb max

# Best practices
* Perform heavy-duty work outside of handler (like DB connections, pulling dependencies)
* Sensitive values use KMS
* Minimize deployment package size to its runtime necessities
* Use layers
* Avoir recursive code: recursive lambda (lambda calling itself)