# Encryption types
* Encryption in flight (SSL)
    * Throught SSL certificates
* Encryption at rest
    * Server side encryption
    * Client side encryption

# KMS: Key Management Service
* KMS Keys us the new name of KMS Customer Master Key (CMK)
* AWS manages encryption keys
* Fully integrated with IAM for authorization
* Easy way to control access to data
* Audit Key usage with CloudTrail
* Can be used in most services (EBS, S3, RDS, SSM, ...)
* Keys also available through API calls
* KMS Keys Types
    * Symmetric (AES-256 keys)
        * AWS services use symmetric keys
        * Never get access to the KMS key unencrypted
    * Asymmetric (RSA & ECC key pairs)
        * Used for Encrypt/Decrypt or Sign/verify operations
        * Public key is downloadable but can't access private key
        * Use case: encryption outside of AWS by users who can't use API
    * Types
        * AWS Owner keys (free): SSE-S3, SSE-SQS, SSE-DDB
        * AWS Managed key: free (aws/service-name, aws/rds, ...)
        * Customer managed keys created in KMS: 1$/month
        * Customer managed keys imported (symmetric key): 1$/month
        * + pay for API call
* Automatic key rotation
    * AWS managed KMS key: 1year
    * Customer-managed key: must be enabled and automatic 1 year
    * Imported KMS key: manual rotation
* Keys scopes
    * are at region level by default
    * now can be cross regions
    * Use case to cross region: copy snapshot (policy need to be set)
        1. Take a snapshot of encrypted snapshot (will be encrypted with same key)
        2. Reencrypt copy with other key in other region (AWS manage the process)
* Key policies
    * Control access to KMS keys: similar to S3 bucket policies
    * Cannot control access without them
    * Default policy
        * Created if no provide a specific policy
        * Complete access to key for any user in the account
    * Custom policy
        * Define users, roles can access key
        * Define who administer key
        * Can allow cross account access
* Using API, secret are max of 4kb. To overcome this limitation, use Envelope Encryption
    * API that will help is GenerateDataKey API
    * Process is
        * Request a Data key with this API
        * AWS sends plaintext data key (DEK)
        * Encrypt in client side the file
        * With encrypted file, build and envelop contained encrypted file with encrypted data key using chosen CMK. AWS sends encrypted DEK. It gives envelope file
    * To decrypt
        * Call decrypt API to decrypt DEK
        * Receive plaintext DEK
        * Decrypt file
    * But it is complex process. Use SDK instead
        * Exists as CLI tool
        * Exists for Java, Python, C, Javascript
    * Data key caching
        * Avoir calling AWS many times
        * Trade-off: can be a security breach
        * Use LocalCryptoMaterialsCache to define max age, max bytes, max number of messages
    * Can use GenerateDataKeyWithoutPlaintet to generate a DEK to use at some point (not immediately)
* Limits & Quotas
    * When exceed request quota get ThrottlingException
        * To respond, use exponential backoff
    * For cryptographic operations, they share a quota
        * Includes requests made by AWS on your behalf (ex: SSE-KMS)
    * Can request a Request Quotas increate through API or AWS support

## S3 bucket key for SSE-KMS encryption
* New setting to decreate API calls and cost by 99%
* Leverage data keys
    * A S3 bucket key is generated
    * Used to encrypt objects
* But less CloudTrail events

# CloudHSM
* KMS -> AWS manages software for encryption
* CloudHSM -> AWS pvosisions encryption hardware
* Dedicated Harware (HSM = Hardware Security Module)
    * AWS manages hardware
* You manage your own encryption keys entirely and also permissions
* HSM device is tamper resistant (FIPS 140-2 Level 3 compliance)
* SUpports both symmetric and asymmetric encryption
* No free tier
* Must use CloudHSM client software
* Redshift supports CloudHSM for database encryption and key management
* Good option to use with SSE-C encryption (client side)
* Clusters are spread accross multi AZ
* Integration with other service through AWS KMS
