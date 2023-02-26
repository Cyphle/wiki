# What is CloudWatch ?

Service that allows:
* Metrics: Collect and track key metrics
* Logs: Collect, monitor, analyze and store log files
* Events: Send notifications
* Alarms: React in real time to metrics/events

# Notes

* Metrics for every service
* Metrics: CPUUtilization, NetworkIn, ...
* Metrics belong to namespaces
* Dimension is an attribute (instance id, environment, ...). Up to 30 dimensions per metric
* Have timestamps
* By default EC2 metrics every 5min
* With detailed monitoring, every 1min
* Free tier allows to have 10 detailed monitoring metrics
* EC2 memory usage is by default not pushed (need to use a custom metric)

# Custom metrics

* Example: ram usage, disk space, logged users
* Have to use api call PutMetricData
* Metric resolution (StorageResolution)
    * Standard: every 1min
    * high resolution 1/5/10/30s (higher cost)
* Accept metric data points two weeks in the past and two hours in the future (have to configure instance time correctly)

# Logs

* Store logs
* Log groups: name, usually an application
* log stream: instances with application / log files / containres
* Log expiration policy (never expires, 30 days, ...)
* Can be sent to:
    * S3
    * Kineses Data Stream
    * Kinesis Data Firehose
    * AWS Lambda
    * Opensearch
* Send logs with SDK, cloudwatch logs agent, cloudwatch unified agent
* Logs from everything: beanstalk, ecs, aws lambda, vpc flow, api gateway, cloudtrail, route53
* Metric filter
    * For example: filter on specific IP
* With filters, can count and trigger cloudwatch alarms
* Insights used to query logs and add queries to cloudwatch dashboards
* Logs subscription filter use for service to listen to logs
* Can do log aggregation from multiple accounts and multi regions (ex: with kinesas data streams and firehose)

# CloudWatch Logs for EC2

* By default, no logs from EC2 to CloudWatch, need to run an agent. Need IAM role
* Agent can run on premise servers
* CloudWatch logs agent is old version and send only to cloudwatch logs
* CloudWatch unified agent can collect additional system level metrics such as ram, processes
    * Centralized configuration with SSM
    * CPU, disk metrics, ram, netstat, processes, swap space

# Logs Metric filter

* Use filter expressions
    * Find specific IP
    * Count error
    * Trigger alarms
* Are not retroactive

# Alarms

* Notifications for any metric
* 3 states
    * OK
    * INSUFFICIENT_DATA
    * ALARM
* Period: How to long to evaluate the metric
* Targets:
    * Stop, terminate, reboot, recover an EC2 instance
    * trigger auto scaling action
    * Send notification to SNS
* Composite alarm
    * When want to use multiple metrics
    * AND and OR conditions
* Can test alarms with cli:
```
aws cloudwatch set-alarm-state --alarm-name "myalarm" --state-value ALARM --state-reason "testing purposes"
```
