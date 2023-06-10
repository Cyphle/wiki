# RDS Aurora

# Notes
* Storage automatically grows in increments of 10Gb
* 15 read replicas
* 20% expensive
* 6 copies accross 3 AZ
    * 4 coppies for writes
    * 3 copies for read
    * Storage is striped across 100s of volumes
* One master
    * If fail, less than 30s of failover
* Cross region replication
* Up to 128 To
* Reader and writer endpoint : load balanced for read endpoint
* Automated patching with 0 downtime
* Has read replica autoscaling
* At rest encryption
    * Has to be defined at launch time
    * If master is not encrypted, read replicas cannot be encrypted
* In flight encryption
    * TLS ready by default
    * Use root certificates
* IAM authentication
    * Can use IAM roles to connect to your database instead of username/pw
* Can have security groups
* Can send logs to cloudwatch
