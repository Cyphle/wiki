# What is RDS

Pour Relational Database Service.

# Pricing
* Pay as you go model. Par heure
* On dmande : pay per hour
* Reserved instance (1 ou 3 ans) : il y a des réductions
* Paie par I/O per millions requests
* Paie pour les transferts de data
* 1 credit = 100% CPU utilization for one minute
* Networking cost
    * Normally cost when data goes from one AZ to another
    * For RDS read replicas in same region, no fee
    * For RDS read replicas in different region, some fee

# Class & types
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

# Parameter groups
* Define configuration values specific to selected DB engine (specific for PostGres, MySQL, etc)
* Default parameter group cannot be edited
* To make changes have to create new group
* New groups inherits from default
* Can be applied region wide
* Examples:
    * autocomit
    * time_zone
    * force_ssl
    * default_storage_engine
    * max_connections
* Dynamic parameters are applied immediatly (can produce downtime)
* Static parameters require manual reboot
* DB parameter group show a status of pending-reboot after applyon change. Status pending-reboot not trigger automatic reboot. Then change to in-sync

# Option groups
* For configuration of optional features offered by DB engines and not covered by parameter groups
* Default option group is empty and cannot be modified
* Have to make new option group.
* Examples:
    * SQLSERVER_BACKUP_RESTORE
    * NATIVE_NETWORK_ENCRYPTION

# Security
* Il faut attacher un security group
* Il est possible d'exposer la base sur internet mais ce n'est pas conseillé
* Pour l'authentification, il est possible d'utiliser plusieurs modes comme password ou IAM

## Network
* Always launch in a VPC to restrict access from internet
* EC2 can be in public subnet and access to private subnet of RDS
* Cannot change VPC after creating
* RDS use security groups to control which IP/security group can communicate with RDS
* Network security is a shared responsiblity between customer and AWS

## IAM
* Use IAM to secure access to RDS resources
* IAM policies to control who can manage through RDS API. Who can create, access or delete resources
* Can use user/password to log in the database
* Can use IAM-based authentication to login to MySQL and PostgreSQL. Token is available for 15min. Obtain it from RDS service. Connection is encrypted with SSL. Can use IAM roles for easy integration with EC2
* Grant least privilege to groups/users/roles
* Can use MFA for sensitive operations
* Can use policy conditions to restrict access to IP, specific date or to require SSL/MFA

## Encryption
* In transit, use SSL/TLS
    * Clients must trust AWS Root CA (have to download certificate from AWS)
    * Enforce SSL: for example with Postgresql rds.force_ssl=1
* At rest, support AES-256 encryption
    * Use KMS
    * Encrypt master and read replicas
    * Must defined at launch time
* Canot encrypt existing unencrypted DB nor cannot create encrypt read replica from unencrypted instance
    * To encrypt an unencrypted snapshot, take a snapshot, copy with encryption and then restore

## Credentials rotation
* It is a best practice
* Use AWS secrets manager
* Supports automatic rotation of secrets
* Secrets maanger provides a lambda rotation function to populate it automatically with ARN in the secret
* Integrates with RDS for MySQL, PostgreSQL and Aurora

# Backups & snapshots
* Can copy full backups and snapshots
* Can copy cross account : share snapshot and then copy
* Can copy cross regions
* When encrypted, copy snapshot with custom key, then share key and then share snapshot (default KMS key cannot be shared)
* Snapshots cannot be shared with some options like TDE (transparent data encryption for Oracle)
* Restored cluster gets applied with
    * New security group
    * Default parameter group
    * Option group that was associated with the snapshot
* PITR recovery
    * Can only restore to a new instance
    * Backup retention period controls the PITR window
    * Can restore to any point in time during your backup retention period
    * RDS uploads DB transaction logs to S3 every 5min
* Exporting to S3
    * Runs in background
    * Doesn't affect performance
    * Exported in Apache Parquet format (compressed and consistent)
    * Allows to analyse in Athena or Redshift

## Backups
* Supports automatic backups
* Capture transaction logs in real time
* Can specify backup window
* Can modify retention period to 35 days
* First backup is a full backup and next are incremental
* Stored in S3 owned and managed by RDS (cannot see them in console)
* Can experience brief IO suspension
* Recommended to use multi AZ option to avoid performance issues
* Integrates for AWS backup service for centralized
* Support PITR (point in time recovery)
* Can only restore a new instance of RDS. To keep the same name, have to rename existing one

## Snapshots
* Manually created
* Full backup, no incremental
* Retention has no limit
* Does not support PITR
* Can use lambda tu take periodic backups and another one to move to S3

## Multi AZ
* High availability, data durability, fault tolerance
* Not used for scaling because second instance is passive (standby)
* SYNC replication to standby
* Automatic failover when planned or unplanned outage of master
* Uses DNS routing to point to master and new master so no need to update connection string
* Failover times (RTO) are typically 60-120s
* Automatic backups are taken from standby and not master anymore (to optimise performance)

## Read replicas
* Read only copy
* To segregate read workload
* Up to 5 read replicas
* Within AZ, cross AZ or cross region
* ASYNC replication so reads are eventually consistent
* Master still offers read possibility
* Different connection string between read and write
* Read replica can be promoted to primary (manual process)
* Must enable automatic backup to use read replicas
* A read replica can have its own standby instance
* Use cases:
    * Need for analytics, only read is needed
    * Performance
* When promoting a read replica to a master, it will be detached of process of replication and works as standalone instance
* Cannot promote a read replica when backup is in progress
* Use cases of promotion:
    * Disaster recovery
    * Avoir performance penalty of DDL operations (like rebuilding indexes)
    * Sharding
* Enabling writes on a read replica for MySQL/MariaDB, set read-only = 0 for read replica
* Capabilities:
    * Can use snapshot to perform PITR of a read replica
    * Can create replica from an existing replica (sync will be master -> replica 1 -> replica 2)
* By default, backups are disabled for read replicas

# Notes
* One RDS instance can have one or more DBs
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
* Can activate automatic upgrade and specify a maintenance window
* Deletion protection is to avoid to delete the database
* No DB patching, no OS patching, AWS takes care of it
* Pour se connecter depuis son poste (exemple avec PostGreSQL), une manière:
    1. CREATE USER <user>
    2. GRANT rds_iam to <user>
    3. `wget https://s3.amazonaws.com/rds-downloads/rds-ca-2019-root.pem`
    4. `export PGPASSWORD="${aws rds generate-db-auth-token --hostname=<endpoint> --port=5432 --username=<user> --region <region>}"`
    5. Connect with some tool passing the certificate



* Automatically modify storage if
    * Free storage is less than 10% of allocated storage
    * Low torage lasts at least 5min
    * 6 hours have passed since last modification
* Within AZ, Cross AZ or cross region
* Read replicas are eventually consistent
* Replicas can be promoted to their own database
* Multi AZ is common for disaster recovery
    * Add a standy database read replica : cannot be used as is but only in case of disaster
* From single AZ to multi AZ is 0 downtime
* Il y a une option free tier pour s'entrainer
* For high availability, active multi AZ.
