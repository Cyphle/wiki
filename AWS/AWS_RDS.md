# What is RDS

Pour Relational Database Service.

# Pricing
* Pay as you go model. Par heure
* On dmande : pay per hour
* Reserved instance (1 ou 3 ans) : il y a des réductions
* Paie par I/O per millions requests
* Paie pour les transferts de data
* 1 credit = 100% CPU utilization for one minute

# Notes
* AWS RDS est compatible avec : PostGreSQL, MariaDB, MariaDB, Oracle, SQL Server, Aurora
* Il s'agit d'un service managé
* Les instances sont lancés dans un VPC
* Le stockage est sur des EBS (gp2 ou io1). Peut être augmenté via de l'auto scaling
* Les backups sont automatisés avec des point in time recovery. Les backups ont une durée de vie. Il est également possible de faire des snapshots et de les restaurer cross region.
* Le monitoring est intégré via CloudWatch. Il y a des Monitoring dashboards
* Des événements sont lancés lorsqu'il y a des opérations ou des problèmes. Il est possible d'être notifié via SNS.
* Peut-être déployé cross AZ.
* Multi AZ est synchrone et les read replica sont asynchrones.
* Automated provisionning, OS patching est automatique
* Supporte le scaling vertical et horizontal
* Cannot SSH
* Scales automatically when storage is at limit. Have to set maximum storage threashold.
* Instance class
    * Standard
    * Memory optimized
    * Bustable performance : small workloads. CPU on demand.
* Storage type
    * General purpose storage (cost effective SSD)
        * Choose storage size
        * Baseine of 3 IOPS/GB
        * Volumes below 1Tb and can burst to 3000 IOPS
        * Variable workloads
        * Used for small to medium sized DB for non prod
* Provisioned IOPS (high performance storage, recommanded for production)
    * Choose storage size and required IOPS
    * Fast and predictable performance
    * Up to 32000 IOPS max per DB instance
    * Use when IO intensive workloads
    * Use when write heavy workloads



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
