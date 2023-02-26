# What is API Gateway ?

* Used as proxy to route calls to services like lambda
* Create public REST API

# Notes
* Support WebSocket protocol
* Handle versioning
* Multiple environment (dev, test, ...)
* Security
* Create API keys
* Handle throttling
* Has Swagger/Open API, import to define APIs and export
* Transform and validate requests and responses
* Generate SDK and API specifications
* Cache API responses
* Integration with
    * Lambda
        * Invoke functions
        * Easy way to expose REST API
    * HTTP
        * Expose HTTP endpoints in backend
        * Add rate limiting, caching, authentication, API keys
    * AWS service
        * Expose any AWS API
* Endpoint types
    * Edge optimized (default)
        * Requests are routed through CloudFront edge
        * API Gateway still lives in only one region
    * Regional
        * Can manually combine with CloudFront
    * Private
* User authentication
    * IAM roles: useful for internal applications
    * Cognito
    * Custom IDP
* Can manage custom domain name HTTPS with AWS Certificate Manager (ACM)
    * If using Edge optimized, certificate must be in us-east-1
    * If using regional, certificate must be in API Gateway region
    * Must set CNAME or A-alias record in Route 53

# Deployment stages
* Making changes in API gateway does not mean they're effective
* Need to make a "deployment" to be in effect
* Changes are deployed to "Stages" like dev, test, prod
* Each stage has its own configuration parameters
* Can rollback a deployment in stage
* Can manage versions of API (v1, v2, ...) with stages
* Stage variables
    * Like environment viriables
    * Can be used in
        * Lamda function ARN
        * HTTP endpoint
        * Parameter mapping templates
    * Are passed to the context object of lambda
* Can define
    * Cache
    * Method throttling
    * WAF
    * Certificate
    * CloudWatch logs
    * Access logging
    * X-Ray
* Can do canary

# Canary deployment
* Can enable canary deployments for any stage
* Choose % of traffic
* Matrics & Logs are separate
* Can override stage variables for canary
* Equivalent of blue green deployment

# Integration types and Mappings
* Integration type MOCK
    * Returns a response without sending the request to the backend
    * Mock call
* Integration type HTTP / AWS lambda & services
    * Must configure integratin request & response
    * Setup data mapping using mapping templates for request & response
* Integration type AWS_PROXY
    * Incoming request from client is the input to lambda
    * Function is responsible for logic of request/response
    * No mapping template, headers, query string, parameters are passed as arguments
* Integration type HTTP_PROXY
    * No mapping template
    * HTTP request is passed to backend
* Mapping templates
    * Modify request/response
    * Rename/Modify query string parameters
    * Modify body content
    * Add headers
    * Uses Velocity Template Language (VTL)
    * Filter output results
    * Example: 
        * integrate with SOAP API (transform SOAP to REST)
        * query string parameters mapping

# Swagger & Open API spec
* Common way of defining REST APIs using API definition as code
* Import existing Swagger / Open API 3.0
    * Method
    * Request
    * Integration request
    * Response
    * + AWS extensions for API gateway and setup
* Can export current API as Swagger / Open API
* Swagger can be written in YAML or JSON

# Cache
* Can be encrypted
* Size from 0.5Gb to 237Gb
* Expensive
* Invalidation
    * Flush from UI
    * Clients with header Caache-Control: max-age=0 (with IAM authorization)

# Usage plans & API Keys
* Usage Plan
    * Who can access one or more API stages and methods
    * How much and how fast can access them
    * Uses API keys to identify clients and monitor
    * Configure throttling limits and quota
* API Keys
    * Securely use API Gateway
    * Can use with usage plan
    * Set throttling limits and quota
* To configure
    1. Create API
    2. Generate or import API keys
    3. Create usage plan
    4. Associate API stages with API keys
* Callers must set header X-API-KEY to use API Keys
* Can sell API in AWS marketplace

# Monitoring, Logging & Tracing
* Can send to CloudWatch logs
* Can override basic settings of level
* Log containers about request and response
* Can use X-Ray
* Monitoring with CloudWatch metrics
    * Are by stage
    * Have cache monitoring
        * CacheHitCount
        * CacheMissCount
    * IntegrationLatency: time to relay a request to backend
    * Latency: time to receive a request and send back (includes IntegrationLatency)
* 29s is the max for an API Gateway to respond
* Errors are 4XXError (client-side) and 5XX (server-side)
* Account limits
    * By default 1000 request per second across all API
    * Soft limit can be increased upon request
    * In case of throttling => 429 Too Many Requests
    * Can set Stage limit & Method limit to improve performance and avoid to use limit on given stage
    * Or define usage plan

# CORS
* Must be enabled to be called from another domain
* OPTIONS pre-flight request must contain headers
    * Access-Control-Allow-Methods
    * Access-Control-Allow-Headers
    * Access-Control-Allow-Origin
* As integration type proxy does not allow API gateway to modify headers, CORS has to be set on backend

# Authentication & Authorization
* IAM permissions can be used to authorize (with IAM policy)
    * Good to provide access within AWS (EC2, lambda, IAM users, ...)
    * Leverages "Sig v4" capabilities where IAM credentials are in headers (API Signed)
    * Authentication is in IAM and Authorization in IAM
    * Can be combined with Resource policies (similar to Lambda resource policies)
        * Define who and what access
        * Main purpose is to define cross account access (combiend with IAM security)
        * To allow only for a VPC endpoint
        * To filter
* Cognito user pools
    * Then API Gateway verifies identity automatically with Cognito
    * Authentication in Cognito and Authorization in API Gateway Methods or backend
* Lambda authorizer (formerly Custom Authorizer)
    * Token based authorizer (ex: JWT or OAuth)
    * Lambda must return IAM policy for the user
    * Authentication is external and authorization is in lambda
    * Example:
        1. Client authenticate with external IDP
        2. Request sent with token
        3. API Gateway call a lambda authorizer that check token with external IDP
        4. Lambda has to return (or create) an IAM principal and policy in response to API Gateway

# HTTP API vs REST API
* HTTP APIs
    * low latency
    * No data mapping (private integration and HTTP proxy)
    * Supports OIDC and OAuth2, CORS
    * No usage plan and API keys
* REST APIs
    * All features except Native OpenID Connect/OAuth2

# Websocket API
* Is a two-way interactive communication between web browser and server
* For stateful use cases
* On first connection a lambda is invoked (called onConnect)
* When something sent invoke a new lambda (sendMessage)
* When disconnect another lambda (onDisconnect)
* Connecting to the API
    * Invoke lambda with connectionId (can be stored in DynamoDB for example)
* During connection
    * Use frames to send messages
    * Still send connectionId
* Server to client messaging
    * Use connection URL callback
* Websocket API routing
    * Incoming messages are routed to different back
    * If no routes, sent to $default
    * Need to request a route selection expression to select the field on JSON to route from
    * Result is evaluated against route keys available in PAI gateway
    * Route is then connected to backend
    * Based on a route table

# Architecture
* Create a single interface for all microservices
* Use API endpoints with various resources (even with ELB)
* Apply domain name and certificates
* Can apply forwarding and transformation rules