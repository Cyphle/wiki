# What is Beanstalk

Il s'agit d'une interface graphique au-dessus de CloudFormation pour créer et déployer des environnements et des applications facilment.

# Notes
* Principes
    * Gives centric view of deploying an application
    * Use all components of AWS
    * Manages : capacity, load balancing, scaling, application health
    * Service is free
* Components
    * Application : collection of components (environments, versions, confs)
    * Application version
    * Environment : collection of AWS resources
        * Tiers : 
            * web server environement tier : typically with ELB
            * worker tier : typically no clients accessing, using sqs. Scale based on sqs messages
        * Can create multiple environment
* Flow
    * Create application -> uplaod version -> launch environement -> manage environement
* Supports : go, java, node, docker, … and can create own custom platform
* Deployment modes
    * Single instance (for dev env)
        * 1 elastic IP, ec2 instance, rds master
    * High availability with load balancer
        * 1 rds on standby
    * Options for updates
        * All at once : fastest but instance are not available
            * Fastest
            * Has downtime
            * No additional cost
        * Rolling
            * Application running below capacity
            * Set bucket size for size of rotation
            * No additional cost
            * Both versions are running
            * First stop bucket size of instances and then start new ones
        * Rolling with additional batches : spins up new instances to move the batch
            * Small additiona cost
            * First add bucket size and remove old ones
            * Good to keep capacity
        * Immutable : deploy in a new asg and then swap all instances
            * Zero downtime
            * High additional cost
            * Longest deployment
            * Quick rollback
            * Fun a first instance in new temporary asg to check if ok and then launch others
            * Move new instances to old asg and kill temporary asg
        * Blue green : create new environment and switch when ready
            * Not an option but can be a strategy
            * Very manual
            * Use beanstalk swap url to change environment after done
        * Traffic splitting : canary testing
            * Use ELB to route percentage of traffic to new asg
* CLI is available with many command to create, check, terminate, etc
* App version are uploaded into zip (uploaded in S3)
* Lifecycle policy
    * Can store at most 1000 versions
    * To clean old versions use lifecycle policy
        * Based on time
        * On space used
    * Can choose to not delete bundled S3 to prevent data loss
* Extensions
    * All parameters can be configured in file
    * File must be in .ebextensions folder in root of project
    * File must end with .config
    * All stuffs in extension files are deleted when application is deleted
* Under the hood it uses cloud formation
    * Possible to use cloud formation resources in .ebextensions
* Cloning
    * Can clone existing environment
* How to perform a beanstalk migration
    * Cannot change ELB type after creating an environment
    * To migrate, clone without ELB
    * Do not provision RDS with beanstalk for prod as it is linked to beanstalk lifecycle
* With docker
    * Provide
        * Dockerfile
        * Dockerrun.aws.json which describes image, port, volumes, logging, etc
    * When single container, beanstalk does not use ECS
    * When running multiple containers will create ecs cluster, ec2 instances, load balancer, task definition and execution
* HTTPS
    * From console
    * From .ebextensions/securelistener-alb.config
    * With ACM or cli
    * Must have a security group
    * Can have a redirec http -> https
* Web server vs worker environment
    * If tasks take time, can offload to worker environment
    * Can define periodic tasks in cron.yaml
* For custom platform
    * Descibe AMI in Platform.yaml
    * Use packer to create AMI
