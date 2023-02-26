# What are quotas

Ce sont les limites offertes par les services

# Notes
* API Rate limits : how many time can call an API
    * EC2 has 100 calls per second
    * S3 GetObject has 5500 per second per prefix
* Service Quotas (service limits)
    * OnDemande standard instances : 1152 vCPU
    * Can open a ticket to increase quotas
* Exponential backoff
    * If get ThrottlingException
    * Retry mechanism included in SDK
    * Must implement yourself if using AWS API as-is
