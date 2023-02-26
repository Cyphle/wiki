# What is VPC

Pour Virtual Private Cloud.

Il s'agit d'un réseau virtuel associé à une région. Il y en a un par défaut.

```
=> REVOIR le cours VPC dans la formation Solution Architecte Associate
```

# Notes
* Private network
* Regional resource
* Subnets to partition VPC defined at AZ level : manage with CIDR
* How to partition
    * Create a public subnet which can access internet
    * Create private subnets which cannot be accessible from internet
* To define access to internet : use route tables
* Internet gateway :
    * Helps VPC to connect to internet
    * Public subnets have route to the internet gateway
* NAT gateway (AWS managed)/NAT instance (self managed)
    * Allow instances in private subnets to access internet while remaining private
    * Create NAT gateway and create a route from private subnet to NAT gateway
* Network ACL (NACL)
    * Firewall which controls traffic from and to subnet
    * Can have ALLOW and DENY rules
    * Attached at subnet level (for public subnet)
    * Include IP addresses
    * Stateless : request that comes in is not allowed to go out by default
* Security groups
    * Firewall controls traffic to ENI or EC2 instance
    * ALLOW rules
    * Statefull : request that comes in can go out
* VPC flow logs
    * Capture IP all traffic going into interfaces
        * VPC
        * Subnet
        * ENI (Elastic Network Interface)
        * For managed interfaces : ELB, ElastiCache, etc
        * Go to S3 and CloudWatch logs
* VPC Peering
    * Connect 2 VPC privately using AWS network
    * Behave as they are in same network
    * Have no overlapping CIDR
    * Is not transitive
* VPC endpoints
    * Allow to connect to AWS services using a private network
    * Exemple : an EC2 in private subnet wants to connect to S3 or dynamoDB
    * VPC endpoint gateway only for S3 and DynamoDB, VPC endpoint interface for other services
    * VPC endpoint interface can be used to connect to cloudwatch
* Site to site VPN
    * Connect to on premise VPC to AWS
    * Encrypted
    * Goes over the public internet
* Direct Connect (DX)
    * Same purpore as site to site
    * It is physical connection
    * Private, secure and fast
    * Takes at least a month to establish
* Three tier architecture exemple
    * Route 53 route to ELB which is in public subnet which redirect to private subnets
    * Data subnet which contains data solutions as RDS, ElasticCache which talks to private subnet
* Example : LAMP Stack on EC2
    * Linux for EC2
    * Apache web service
    * MySQL on RDS
    * PHP application logic
