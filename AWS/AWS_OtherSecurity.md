# CloudWatch log encryption
* Can be encrypted with KMS
* Encryption at log group
* Cannot associate CMK with CloudWatch console (web interface)
    * Must use API
* Can encrypt existing logs

# CodeBuild security
* Secrets in CodeBuild
    * Do not store them as plaintext in environment variables
    * Instead reference SSM or Secret Manager

# AWS Nitro Enclave
* When process highly sensitive data in an isolated compute environment like healthcare, financial, ...
* Nitro Enclave is fully isolated virtual machines, hardened and highly constrainted
    * Not a container, not persistent storage, no interactive access, no external networking
* Helps reduce attack surface for sensitive data 
    * Cryptographic Attestation: only authorized code can be running
    * Only Enclaves can access data (integration with KMS)
* Launch EC2 instance compatible and set EnclaveOptions to true