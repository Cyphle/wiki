# What is CI/CD ?

CI : Continuous integration. Integration continue du code dans le main code through phases like testing
CD : Continuous deploymet

# CodePipeline

* Automating our pipeline from code to Beanstalk
* Visual tool to orchestrate CICD
* Example phases:
    * Source
    * Build
    * Test
    * Deploy
* Each pipeline stage create artifacts stored in S3
* Can use CloudWatch Events and EventBridge to see what is happening
* CloudTrail can be used to audit AWS API calls
* Requires an IAM role to do things
* Build phase can be delegated to Jenkins or to CodeBuild
* Action groups are actions that are launched in a stage. go with stages (manual approval, or other event)

# CodeBuild

* Is outside VPC but can specify a VPC
* Building and testing code (aka Jenkins)
* Source can be CodeCommit, S3, Github, BitBucket
* Build instructions in file buildspec.yml. File must be at the root of project
* Output logs can be in S3 & cloudwatch
* Build projects can be defined in CodePipeline or CodeBuild.
* Java, ruby, etc and can be extended with docker images
* Build in container and can use custom image (it uses containers in background to run)
* buildspec.yml
    * can define env variables from plaintext, SSM, AWS secret manager
    * phases: install, pre_build, build, post_build
    * artifacts : what to upload to S3 (ecnrypted with KMS)
    * cache: files to cache to speedup next builds
* Can run CodeBuild in local on desktop, need CodeBuild Agent
* Can be launched in VPC. By default containers are launched outside VPC
    * Specify VPC ID, subnet IDs, security group IDs
    * Then can use resources in VPC (RDS, ElastiCache, EC2, ALB, ...)
    * Use case in VPC are integration tests, data query, internal load balancer, ...
* In buildspec.yml can use linux commands

# CodeDeploy

* Deploying to EC2 (not Beanstalk) or on premise
* Ways to handle deployments : Ansible, Terraform, Chef, Puppet, ... but can also use AWS CodeDeploy
* Server must run CodeDeploy Agent
* Need appspec.yml file at root in code source to tell how to deploy
* Code is pulled from S3 or Github
* Components
    * Application: a unique name functions as a container (revision, config, ...)
    * Compute plateform EC2/on premise, lambda, ECS
    * Configuration
        * EC2/On premise: specify min healthy instances
        * AWS Lambda/ECS : specify how traffic is routed
    * Deploymeng group
        * group of tagged EC2 instances
        * allow to deploy gradually
    * Deployment type
        * In-place : for EC2/On premise. basic deployment. Stop, deploy, start
        * Glue/Green : EC2, lambda, ECS
    * IAM instance profile
        * give EC2 instances the permissions to access S3 or Github
    * Application revision
        * application code + appspec.yml
    * Service role : IAM role to perform operations on EC2, ASG, ELB, ...
    * Target revision: most recent revision you want to deploy to a group
* appspec.yml
    * files: how to source and copy source code to filsystem (source and destination)
    * hooks: instructions to deploy new version. Order of hooks:
        * ApplicationStop
        * DownloadBundle
        * BeforeInstall
        * Install
        * AfterInstall
        * ApplicationStart
        * ValidateService (important hook to check deployment)
        * Hooks can launch sh scripts
* Deployment configuration
    * Types
        * One at a time: one EC2 at a time. If one instance fails, deployment stops
        * Halt at a time: 50% then 50%
        * All at once
        * Custom: ex: min healthy host = 75%
    * Failures:
        * EC2 how many EC2 fails to say failed
        * To rollback, redeploy old deployment or enable auto rollback for failures
* Deployment groups
    * Tagged EC2
    * ASG
    * Mix ASG and tags
    * CUstomization in scripts with DEPLOYMENT_GROUP_NAME env var
* It is the instance which pulls the code from S3 and not CodeDeploy
* To deploy in ASG
    * In-place: automatically update instances created by ASG
    * Blue/Green: 
        * a new ASG is created. 
        * choose how long to keep old instances in old ASG. 
        * must be using an ELB
* Rollback & redeploy
    * Automatically: when fails, cloudwatch alarm
    * Manually
    * Can be disabled
    * CodeDeploy redeploy last known working version. It is a new deployment not a restore
