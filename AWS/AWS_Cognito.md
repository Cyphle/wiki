# What is Cognito ?

AWS Identity Provider (IDP)

# Notes
* Cognito User Pools
    * Sign in functionality
    * Integration with API Gateway & Application Load Balancer
* Cognito Identity Pools (Federated Identity)
    * Provide AWS credentials to users to access resources
    * Integration with Cognito User Pools as identity provider
* Cognito Sync
    * Synchronize data from other user databases to Cognito
    * Deprecated and replaced by AppSync

# Cognito User Pools (CUP)
* Create serverless database of user
* Simple login: username (or email)/password
* Password reset
* Email & Phone number verification
* MFA
* Federated Identities: Facebook, Google, SAML, OIDC...
* Block users when credentials compromised
* Use JWT
* Integrates with API Gateway an Application Load Balancer
* Lambda triggers
    * CUP can invoke lambda synchronously on triggers like authentication event, sign-up
* It is an IDP

# Cognito Identity Pools (Federated Identities)
* Get identities for users so they obtain temporary AWS credentials
* Identity pool (identity source) can include
    * Public provider (login with Amazon, Facebook, Google, Apple)
    * Users in Cognito User Pools
    * OIDC, SAM
    * Custom login server
    * Allow for unauthenticated (guest) access
* Users can access AWS services directly or through API Gateway
    * IAM policies applied to the credentials are defined in Cognito
    * Can be customized based on user_id for fined grained control
* Flow
    1. Login from IDP
    2. Exchange token with Identity Pools
    3. Identity pools validate agains IDP
    4. Get temporary credentials using STS
    5. Return credentials
    6. User access services
* With CUP
    * Same flow but IDP is CUP
* IAM roles
    * Can set default IMA roles for authenticated users and guest
    * Define rules to customize based on user ID
    * Can partition users using policy variables
    * IAM credentials obtained by through STS
    * Roles must have "trust" policy

# User Pools vs Identity Pools
* User pools
    * Database of users for applications
    * Allows to federate logins
    * Customize UI login
    * Has triggers
* Identity Pools
    * Allows to obtain AWS credentials
    * Can login with other IDP
    * Users can be guets
    * Users are mapped to IAM roles & policies
