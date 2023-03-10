# What is SAM ?

* For Serverless Application Model
* Framework for developping and deploying serverless applications

# Notes
* In YAML
* Generate complex CloudFormation (built on top of cloudformation)
* Requires transform and resources sections
* Supports anything from CloudFormation: Outputs, Mappings, Parameters, Resources, ..
* Only two commands to deploy to AWS
* Can use CodeDeploy to deploy lambda
* Can help run lambda, API Gateway, DynamoDB locally
* Recipe
    * Transform header indicates it's SAM template
        * Transform: 'AWS::Serverless-2016-10-31'
    * Write Code
        * AWS::Serverless::Function
        * AWS::Serverless::Api
        * AWS::Serverless::SimplateTable
    * Package & Deploy 
        * aws cloudformation package or sam package
        * aws cloudformation deploy or sam deploy
* To run locally, use SAM CLI
    * Provides lambda like environment
    * SAM CLI + AWS Toolkits -> step through and debug code

# Some commands
* `sam init` command to initialize a sam project
* `sam package` to package and upload to S3 and generate template
* `sam deploy` to deploy on CloudFormation
* `sam publish`to publish into SAR

# Application repository
* Repository to use applications generated by other people

# SAM Policty Templates
* List of templates to apply permissions to lambda functions
* Important policy examples
    * S3ReadPolicy
    * SQSPollerPolicy
    * DynamoDBCrudPolicty 

# SAM and CodeDeploy
* Natively uses CodeDeploy to update lambda
* Traffic shifting feature using alias
* Pre and Post traffic hooks to validate deployment
* Rollback using CloudWatch alarms

# Serverless Application Repository (SAR)
* Managed repository for serverless applications
* Applications are packaged using SAM
* Can share publicky or with specific AWS accounts
