# What is RDS

Pour Relational Database Service

# Notes

* PostGreSQL, MariaDB, MariaDB, Oracle, SQL Server, Aurora
* Automated provisionning, OS patching
* Continuous backups and restore
* Monitoring dashboards
* Lulti AZ
* Storage backed by EBD gp2 or io1
* Cannot SSH
* Scales automatically when storage is at limit
* Have to set maximum storage threashold
* Automatically modify storage if
    * Free storage is less than 10% of allocated storage
    * Low torage lasts at least 5min
    * 6 hours have passed since last modification
* Within AZ, Cross AZ or cross region
* Read replicas are eventually consistent
* Replicas can be promoted to their own database
* Networking cost
    * Normally cost when data goes from one AZ to another
    * For RDS read replicas in same region, no fee
    * For RDS read replicas in different region, some fee
* Multi AZ is common for disaster recovery
    * Add a standy database read replica : cannot be used as is but only in case of disaster
* From single AZ to multi AZ is 0 downtime
* Il y a une option free tier pour s'entrainer
* For high availability, active multi AZ.
* Multi AZ is synchronous and read replicas are asynchronous
* RDS run in VPC

## RDS Aurora
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
