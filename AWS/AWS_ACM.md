# What is ACM ?

For Amazon Certificate Manager

# Notes
* Provision, manage and deploy SSL/TLS certficates
* Free of charge for public TLS certificates
* Automatic renewal
* Integrations with
    * ELB
    * CloudFront
    * API Gateway

# Private Certificte Authority (CA)
* Allows to craete private Certificate Authorities (CA) including root and subordinaries
* Can issue and deploy end-entity X.509 certificates
* Certificates are trusted only by your organization and not internet
* Works for AWS services that are integrated with ACM
* Use cases 
    * Encrypted TLS
    * Cryptographically signing code
    * Authenticate users, computers, API, endpoints, IoT devices
    * Enterprise customers building a public key infrastructure (PKI)
    