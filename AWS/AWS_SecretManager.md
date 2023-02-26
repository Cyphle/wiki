# What is Secret Manager ?

* It is to store secrets

# Notes
* Can force rotation of secrets
* Automate generation of secrets on rotation
* Integration with RDS, Redshift, DocumentDB, ...
* Secret are ecnrypted with KMS

# CloudFormation integration
* Specify `ManageMasterUserPassword: true` and user password will be automatically created and managed
* Or can create separately a secret and use dynamic reference to insert it in RDS configuration

# SSM vs Secret Manager
* Secret Manager
    * is more expensive
    * strong integration with RDS, Redhisft, ...
    * KMS is mandatory
    * Integrate with CloudFormation
    * rotation and can invoke lambda to sync services like RDS
* SSM
    * simple API
    * No secret rotation
    * KMS is optional
    * Integration with CloudFormation
    * Can pull Secret Manager secret from SSM API
