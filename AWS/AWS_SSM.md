# What is SSM Parameter Store ?

* Secure storage for configuraiton and secrets
* Equivalent of Hashicorp Vault

# Notes
* Encryption with KMS
* Serverless, easy SDK
* Version tracking
* Security with IAM
* Notifications with EventBridge
* Integration with CloudFormation
* Use store hierarchy like
```
/level1/level2/key = value
```
* Access to secret with
```
aws/reference/secertsmanager/<SECRET_ID_IN_SECRET_MANAGER>
```
* Public parameters are available like
```
aws/serviceami-amazon-linux-latest/amzn2-ami-hvm-x86_64-gp2
```
* Tiers
    * Standard: 4kb of value, free
    * Advanced: 8kb, not free
* Can assign a TTL to a parameter (expiration date) to force updating or deleting sensitive data
    * Can assign multiple poilicies
