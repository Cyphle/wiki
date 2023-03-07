# Industrialisation des infrastructures

## Checklist
*  Software installation
    * Tools: Bash, Ansible, Docker, Packer
* Software Configuration: configuration, TLS, service discovery, leaders, followers, replication, etc
    * Tools: Chef, Ansible, Kubernetes
* Provisionning: servers, load balancers, network configuration, firewall settings, IAM permissions, etc
    * Tools: Terraform, CloudFormation
* Deploy softwares: roll out updates
    * Tools: ASG, Kubernetes, ECS
* High availability: processes, servers, services, datacenters, regions
    * Tools: multi datacenter, multiregion
* Scalability: servers vertical and horizontal
    * Tools: Auto scaling replication
* Performance: Optimize CPU, memory, disk, network, GPU, benchmarking, load testing, profiling
    * Tools: Dynatrace, Valgrind, VisualVM
* Networking: static and dynamic IPs, ports, service discovery, firewalls, DNS, SSH, VPN
    * Tools: VPCs, firewalls, Route 53
* Security: encryption in transit and at rest, authentication, authorization, secrets management, server hardening
    * Tools: ACM, Let's Encrypt, KMS, Vault
* Metrics: availability, business metrics, app metrics, server metrics, events, observability, tracing, alerting
    * Tools: CloudWatch, DataDog
* Logs: rotation, aggregation
    * Tools: Elastic Stack, Sumo Logic
* Data backup: databases, caches, other data, replication
    * Tools: AWS Backup, RDS snapshots
* Cost optimization: instances, spot, reserved, auto scaling, clean resources
    * Tools: Infracost
* Documentation: code, architecture, practies, playbooks to respond to incidents
    * Tools: READMEs, wikis, slack, IaC
* Test
    * Tools: Terratest, tflint, OPA, InSpec