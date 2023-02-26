# What is ECS

Pour Elastic Container Solution.

C'est un peu un concurrent à Kubernetes propriétaire d'AWS.

# Notes
* Launch docker container = launch ECS tasks on ECS clusters
* Launch types
    * EC2 Launch type
        * EC2 are instances
        * EC2 must run the ECS agent
        * AWS start and stop containers
        * Must provision EC2
    * Fargate : serverless
        * AWS runs ECS task based on CPU/RAM needed
        * To scale, increase number of tasks
        * Spot instances can be used
* IAM roles
    * EC2 launch type : ec2 instance profile
        * Used by ecs agent
        * Logs to cloudwatch logs
        * Pull images from ecr
        * Use ssm
    * ECS task role
        * Specific role per task
        * Each role can be used for different services (ex : role to call s3 and role to call dynamodb)
* Load balancer
    * ELB can be set in front
    * NLB for high throughput or pair with private like
* Data volumes
    * EFS
        * Mount file systems on ECS tasks
* Can create automatically ASG when setup cluster
* A container is launch in an ECS task
    * Create a task
    * Launch a service from a task
    * Also can launch task : like a batch job. Container that terminates after finishing
* To setup an ELB for ECS tasks
    * Need 2 security groups
        * 1 for ELB
        * 1 for ECS tasks : input from ELB
* Can have Application Auto Scaling
    * Criteria
        * CPU utilization
        * Memory
        * ALB request
    * Target tracking : based on target value for cloudwatch metric
    * Step scaling : cloudwatch alarm
    * Scheduling scaling : specified date/time
    * ECS Auto scaling is not EC2 auto scaling
    * For EC2
        * ASG
        * ECS Cluster capacity provider (better way)
* Rolling updates is available
    * Parametered with min and max % of healthy
* Some scenarios
    * Tasks invoked by event bridge : rule to run task on the go
    * With event bridge schedule
    * With SQS queue
* Task definition
    * In JSON to tell how to run container
    * Informations
        * Image name
        * Port binding
            * On EC2 launch type
                * If host port not set, it is set randomly
                * An ALB can use dynamic host port mapping to find ports
            * On Fargate launch type
                * Each task has a unique private IP (there is no host)
                * Can only define host port
                * Set ENI ecs cluster to allow ALB
        * Memory, cpu
        * Env vars
            * Hardcoded
            * SSM
            * Secrets manager
        * Networking
        * Iam role
            * One IAM role per task definition to access another service
            * Each task will have role definition
        * Logging conf (ex : cloudwatch)
        * Can define up to 10 containers
* Data volumes (bind mounts)
    * Share data between containers in the same task definition
    * Works for EC2 and fargate tasks
    * For EC2, use instance storage
    * For Fargate, use ephemeral storage (20Gb to 200Gb)
* Tasks placement
    * For EC2 launch type
        * ECS needs to choose placement depending on cpu, memory and port
    * Does not work for fargate (no need)
    * Task placement strategy
        * Binpack : 
            * based on least available CPU or memory
            * Minimize instances (cost saving)
            * Fill entirely instance
        * Random
        * Spread
            * Place task evenly on specified valy (instance id, az, …)
        * Can be mixed
    * Task placement contraints
        * distinctInstance : no task on same instance
        * memberOf : place satisfying expression (can use cluster query language)
