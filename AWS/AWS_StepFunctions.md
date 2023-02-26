# What is Step Functions ?

Step Functions are used to model workflows as state machines

# Notes
* For
    * Order fulfillment
    * Data processing
    * Web applications
    * Any workflow
* Each step has to do little amount of work

# Task States
* Do something in Step Function
* Can invoke
    * Lambda
    * AWS Batch job
    * Run ECS task and wait for it to complete
    * Insert into DynamoDB
    * Publish to SNS, SQS
    * Launch another step function
* They run an activity
    * EC2
    * ECS
    * On premise
    * Activities can poll step functions for work
    * Activities can send results back to step functions
* States
    * Choice state
    * Fail or succeed state
    * Pass state
    * Wait state
    * Map state
    * Parallel state: begin parallel branches of function

# Error handling
* Any state can encounter runtime errors for example
    * Definition issue
    * Task failure
    * Transiant issue
* se Retry (to retry failed state) and Catch (transition to failure path) in state machinie instead of inside application code
* Predefined error codes
    * States.ALL: matches any error name
    * States.Timeout
    * States.TaskFailed
    * States.Permissions
* The state may report is own errors
* Retry allow to define what happens and how many times to retry
    * Define cases for error cases
    * Evaluated from top to bottom
    * Definition of retry
        * ErrorEquals: match a specific kind of error
        * IntervalSeconds: initial delay before retrying
        * BackoffRate: multiple delay after each retry
        * MaxAttemps
    * When max attemps are reached go to Catch block
* Catch
    * Definition of catch
        * ErrorEquals
        * Next: state to send to
        * ResultPath: a path that determines what input is sent to the state specified in the next field

# Standard vs Express
* Standard workflow has max duration 1year
* Express workflow has max duration 5min
* Other differences can be found in docs: https://docs.aws.amazon.com/step-functions/latest/dg/concepts-standard-vs-express.html
