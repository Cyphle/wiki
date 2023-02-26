# What is EC2

Pour Elastic Cloud Computing.

Ce sont des VM

# Notes

* User data : script launched at startup
* Types
    * General : équilibré CPU, RAM et networking (m series)
    * Compute : pour du CPU (C series)
    * Memory (R series pour Memory)
    * Storage : optimisé pour local storage pour par ex OLTP, cache (I, D, h1 series)
* SSH user ec2-user
* Pour donner des accès à un EC2 à d'autres services, utiliser les IAM Roles
* Purchasing options
    * On demand : billing per second for linux or windows and hour for others
    * Reserved instance : environ 70% de réduction (réservation : type, region tenancy, os). 1 an ou 3 ans
    * Saving Plans : c'est par exemple je veux dépenser 10$/hour pour 1 ou 3 ans
    * Spot instances : environ 90% de réduction mais on définit un prix max à mettre si ça dépasse, l'instance est perdue
    * Dedicated host : physical host reserved
    * Dedicated Instance : reserved hardware
    * Capacity reservation : reserve on demand instances in a specific AZ
* Instance metadata
    * Allows EC2 instances to learn about themselves without using an iam role
    * Use 169.254.169.254/latest/meta-data : internal url
    * Can retrieve iam role name but not iam policy
    * Metadata = info about ec2
    * Userdata = launch script

## EC2 Instance Store
* Hardware disk attached to physical server for better IO perf
* It is ephemeral
* Lost when EC2 is stopped