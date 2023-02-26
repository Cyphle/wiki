# What is S3

Pour Simple Storage Service.

Solution de stockage d'objets au niveau région.

Il s'agit d'une des plus anciennes offres d'AWS. C'est une solution au coeur de beaucoup d'autres et d'architectures.

# Notes
* For back up and storage, disaster recovery, archive, hybrid cloud storage, application storage, media hosting, datalakes & big data, …
* Buckets
    * "Top level directory"
    * Have globally unique name (across all regions and all accounts)
    * Stores as key/value :
        * S3://my-bucket/my_folder/my_file.txt : key is my_folder/my_file.txt
    * Max size is 5To. For upload of more than 5GB must use muti part upload
* Defined at level Region but can be seen globally
* Security
    * User based : IAM policies attached to user
    * Resource based : bucket policies allows cross account
    * ACL : 
        * object access control list : finer grain
        * Bucket access Control list : less common
    * If user has IAM permissions allow it or resource policy allows it AND there'es no explicit deny
    * JSON based policies
        * Resources : buckets and objects (ex : arn:aws:e3:::examplebucket/*)
        * Actions : set of API to allow or deny
        * Principal : account or user to apply the policy
        * Effect : allow or deny
        * Can be used to force encryption at upload
    * For EC2 need to use IAM role which has the policy (roles are for service to service)
    * For public access and cross account access attach policy to bucket
    * Block Public Access can be set at account level and to prevent data leak
* Support versionning
    * When enabled, deleting main object add a delete marker. Markers are deletables
    * Deleting old versions is destructive
* Replication
    * CRR : cross region replication
    * SRR : same region replication
    * Asynchronous replication
    * Can be replicated in different accounts
    * Must give IAM permissions (role)
    * Only new objects are replicated
        * If replication old objects need to use S3 Batch replication
        * Can replicate delete markers (optional)
        * Deletions with a version ID are not replicated (do not delete permanent deletions)
    * No chaining replication
    * Only works when versionning is activated
    * To replicate
        * Create replication rule
* Storage classes are set for each object
    * Standard general purpose
        * By default
        * Frequenty accessed data
        * Low latency and high throughput
    * Infrequent access (IA)
        * Data that is less frequently access but require rapid access when needed
        * Lower cost
        * For disaster recovery and backups
    * One zone infrequent access
        * Same as infrequent access but in same AZ
    * Glacier are for archiving. Pay for storage and retrieval
        * Glacier instant retrieval : 
            * Milliseconds retrieval
            * Minimum storage duration of 90 days
        * Glacier flexible retriebal
            * Expedited (1 to 5min), standard (3 to 5 hours), bulk (5 to 12 hours) for retrieval
            * Minmum storage is for 90 days
        * Glacier deep archive
            * Long term storage
            * Standard (12hours), bulk (48 hours)
            * Minimum storage is 180 days
    * Intelligent tiering
        * Move objects between access tiers
        * Tiers
            * Frequent access tier default (automatic)
            * Infrequent access tier : not accessed for 30 days (automatic)
            * Archive instant access tier : not accessed for 90 days (automatic)
            * Archive access tier (optional) : configurable for 90 days to 700+ days
            * Deep archive access tier (optional) : conf for 180 days ot 700+ days
    * Can set classes manually or use AWS choose (lifecycle rules)
    * Create lifecycle rules to automatically move objects between classes
        * Transition actions : configure objects to transition to classes
        * Expiration actions : delete after some time
        * Rules for certain prefix
        * Rules for tags
    * Possible to transition between storage classes
* Event notifications
    * Created when something append
    * Can be sent to other services like SNS and SQS
    * Can be used in EventBridge to archive, replay events, reliable delivery
    * Need to allow S3 to write into queues by updating policy in queue
* Performances
    * 100 to 200 ms
    * 3500 put/copy/post/delete per s
    * 5500 get head per s per prefix per bucket
    * Multipart upload recommanded for file > 100MB and mandatory for > 5Gb
    * S3 transfer acceleration
        * For upload and download
        * Transfer files to AWS edge location in target region
    * S3 byte range fetches
        * Get specific byte range GET
* S3 Select & Glacier Select
    * Retrieve less data using SQL for server side filtering
* Security
    * Encryption
        * Server Side Encryption
            * SS3 with S3-Managed Keys (SSE-S3) enabled by default => AES-256 (set header x-amz-serve-side-encryption: aes256)
            * With KMS (SSE-KMS) can be used with key usage log using CloudTrail (header : x-amz-server-side-ecnryption: aws:kms)
					® Beware of KMS quota 5500 to 30000req/s based on region
            * With customer provided keys (SSE-C) : send key to aws and use it (use https and pass key in header, keys are not stored)
        * Client side encryption
            * Encrypt client side and upload
    * Encryption in transit
        * Use https endpoint to S3
    * Can enforce encryption using bucket policy (in condition of policy)
        * Policies are prioritaire of default encryption
    * CORS
        * Set target bucket cors policies in setting
    * MFA delete can be activated to permanently delete a file or suspend versioning on the bucket (can only be activated by root account)
* Access logs
    * Any request by any account are logged into another bucket (do not set logging bucket as the same of logged bucket, it will create infinite loop
* Presigned urls
    * Can be generated by console, cli or sdk
    * By console : up to 720min available
    * Cli up to 168 hours
    * Permissions inherit from user that generated the url
* Access points
    * Facilitate access to buckets
    * Connect access points to folders
    * Attach policy on access points (like allow groups)
* Object lambda
    * Use AWS lambda to change the object before it is retrieved by the caller application
    * On top of access point and object lambda access point
    * Bucket -> access point -> lambda -> lambda access point -> user
    * Use cases : adding infos, change format, adding watermark
