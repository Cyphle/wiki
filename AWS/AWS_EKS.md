# What is EKS

Pour Elastic Kubernetes Service.

Service managé Kubernetes.

# Notes

* Supports EC2 launch mode
* Supports Fargate
* Expose EKS service with ELB
* Node types
    * Managed node groups 
        * Parts of ASG
        * Managed by AWS
        * On demand and spot instances
    * Self managed nodes
        * Can use prebuilt eks optimized AMI
    * Fargate
* Data volumes
    * Specify storageclass onn manisfest of EKS
    * Use Container storage Interface (CSI)
    * Support
        * EBS
        * EFS (Fargate uses this only)
        * FSx for Lustre
        * FSx for NetApp ONTAP