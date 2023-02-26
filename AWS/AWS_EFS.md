# What is EFS

Pour Elastic File System

Il s'agit d'une solution de stockage distribuée.

# Notes

* Managed NFS (network file system)
* Mounted on many EC2
* Multi AZ
* Highly available scaleble, expense, pay per use
* POSIX file system (aka linux fs)
* Encryption at rest
* Autoscaling
* Storage classes (lifecycle management)
    * Standard : for frequently accessed files
    * Infrequent access (EFS-iA) : cost to retrieve files
		