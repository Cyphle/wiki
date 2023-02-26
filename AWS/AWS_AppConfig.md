# What is AppConfig ?

Configure, validate and deploy dynamic configurations

# Notes
* Deploy dynamic configuration changes to applications
    * Don't need to restart application
* Provide Feature flags
* Integrates with EC2, lambda, ECS, EKS, ...
* Can monitor with CloudWatch Alarm (useful for rollback)
* Validate configuration changes using
    * JSON schema
    * Lambda function