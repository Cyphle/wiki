# What is CloudTrail ?

* Provides governance, compliance and audit for account
Allows to:
* Perform internal monitoring of API calls
* Audit changes to AWS resources made by users (console, sdk, cli, services)

# Notes

* Logs can go to CloudWatch logs or S3
* Can be applied to all regions (by default) or a single region
* Kinds of events
    * Management events
        * Operations that are performed on resources
        * By default, managements events are activated
    * Data events
        * By default not activated because high volume operations
        * Object-level activity (ex: getobject, deleteobject in S3)
        * Lambda execution activity
    * Insights Events
        * Detect unsusual activity
            * Inaccurate resource provisioning
            * hitting service limits
            * Burst of IAM actions
            * Gaps in periodic maintenance activity
        * Analyse normal management events to create a baseline
        * Continuously analyse write events to detect unusual patterns
            * Anomalies appear in console
            * Event to sent to S3
            * EventBridge is generated for automation needs
* Retention by default is 90 days
* To keep events, log them to S3 and use Athena to analyse
* In dashboard
    * Event history: management events
    * Trail: create tails to send to S3 or cloudwatch logs for example