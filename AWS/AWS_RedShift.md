# What is RedShift ?

* BI database
* Provide powerful queries and analytics
* Easy create views
* Works with nodes to distribute data
    * Mannually provisionned and can have reserved machines
    * Serverless (Elastic Resize)
* Use case is for asynchronous reports on big data

# Nodes
* RA3 : with integrated stockage
* DC2 : high calculus with included SSD

# Some features
* Snapshot
* Recovery points
    * Automatically done every 30mins with retention period of 24h
* User defined functions (UDF)
* Store procedures
* Create views
* Spatial functions
* Federated queries
* Data lake queries
* HyperLogLog
* Querying across databases
* Data sharing across Redshift clusters
* Can store semistructured data and query them
* Machine learning
* SQL commands and functions
* Integrated with CloudTrail
* Can pause cluster to stop paying
* There are SDK to interact with Redshift (Java for instance)

# Reshift Processing Unit (RPU)
* One RPU is 16Gb of RAM
* Default is 128 RPU
* Can set usage limit such as RPUs used per day

# Cost
* Pay also per data transfer
* Pay per use
    * Based on node use
* With multi AZ pay double

# Concepts
* Namespace : collection of database objects and users
* Workgroup : collection of compute resources (RPU, security groups, usage limits)
* Cluster : one or more compute nodes. When > 2 nodes, an additional leader handles externel communication
* Database : cluster contains one or more databases
* Redshift is a relational database management system (RDBMS) and compatible with other RDBMS applications. 