# What is X-Ray ?

Allows to:
* Troubleshoot application performance and errors
* Distributed tracing of microservices

Un peu un équivalent de Zipkin

# Notes

* Visual analysis of applications
* Analyse performance and bottlenecks
* Analyse dependencies in a microservice
* Request behavior
* Errors and exceptions
* SLA
* Compatibility
    * Lambda
    * Beanstalk
    * ECS
    * ELB
    * API gateway
    * EC2 and on premises
* Provide tracing (aka Zipkin)
* Annotations with traces
* Security
    * IAM for authorization
    * KMS for encryption
* Enable 
    * by import AWS X-Ray SDK in code source
    * Install X-Ray as low level UDP packet interceptor
    * For lambda run x-ray deamon
    * Each application must have IAM rights to write data to x-ray
    * Application sends traces to deamon and deamon sends to x-ray by batch
* Capture
    * HTTP requests
    * Database calls
    * Queue calls (sas)
* Segment: element sent by application
* Subsegment: when needing more details
* Trace: e2e segments
* Sampling: decrease amount of requests sent to x-ray to reduce cost
    * By default, record first request each second and 5% of additional requests
    * One request per second is the reservoir
    * 5% is the rate at which additional requests beyond the reservoir size are sampled
    * When changing sampling rules, not needed to restart service
* Annotations: key value pairs used to index traces, used for search
* Metadata: key value pairs not indexed, not used for search
* Deamon can send cross account

# API

* Write APIs
    * PutTraceSegments: upload segment document to AWS X-Ray
    * PutTelemetryRecords: Used by AWS X-Ray deamon to upload telemetry (received counts, connection errors, ...)
    * GetSamplingRules: all sampling rules (to know what/when to send)
    * GetSamplingTargets & GetSamplingStatisticSummaries: advanced
    * Deamon needs right IAM policy
* Read APIs
    * GetServiceGraph: get main graph
    * BatchGetTraces: retrieve traces specified by ID
    * GetTraceSummaries: retrieves IDs and annotations for traces available for a specified time range or pass IDs to get batch
    * GetTraceGraph: retrieves a service graph by ID

# With Beanstalk

* Beanstalk include X-Ray deamon
* Run deamon by setting an option in .ebextensions/xray-deamon.config
```
options_settings:
    aws:elasticbeanstalk:xray:
        XRayEnabled: true
```
* Deamon is not provided when running multiple container, need to manage yourself

# With ECS

* One solution: Use X-Ray container as a deamon in EC2 instances
* Another solution: Use sidecar pattern and run a sidecar container with each container
* With Fargate must use sidecar pattern