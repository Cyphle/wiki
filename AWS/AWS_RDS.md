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

## Credentials rotation
* It is a best practice
* Use AWS secrets manager
* Supports automatic rotation of secrets
* Secrets maanger provides a lambda rotation function to populate it automatically with ARN in the secret
* Integrates with RDS for MySQL, PostgreSQL and Aurora

# Backups & snapshots

# Notes
* One RDS instance is one instance of database
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
