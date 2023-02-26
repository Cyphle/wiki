# What is Route 53

Il s'agit d'un service de DNS en plus d'un registrar.

Permet donc d'acheter des domaines et de les gérer.

# Notes
* Highly available full managed and authoritative DNS
* Authoritative : customer (me) can update the DNS records
* Domain registrar
* Abality to check health of resources
* Records : define how to route traffic for a domain
    * A : map hostname to IPv4
    * AAAA : map hostname to IPv6
    * CNAME : map hostname to another hostname (target must be A or AAAA, cannot create cname for top node of DNS namespace Zone Apex)
    * NS : name servers for the hosted zone
    * Alias (specific to route 53) : points a hostname to aws resource (app.mydomain.com => blabla.amazonaws.com)
        * Works for root domain and non root domain
        * Free of charge
        * Native healthcheck
        * Exemple route to ELB
        * Targets :
            * ELB
            * Cloudfront
            * API Gateway
            * Elastic beanstalk
            * S3 websites
            * VPC interface endpoints
            * Global accelerator
            * Route 53 in same hosted zone
        * Cannot set to EC2 DNS name
    * And others CAA/DS/…
* Hosted zones
    * Container fo rrecords that define how to route traffic to a domain and subdomains
    * Public hosted zone
        * Records specify how to route traffic on the internet
        * 0.50/month per hosted zone
    * Private hosted zone
        * Inside a vpc
* Pay for traffic to route 53, so set a right TTL
* Policies
    * Simple : route to single resource. Can have multiple values, if so a random one is chosen. No health check. Alias to aws resources
    * Wieghted : % of requests to go to specific resource
    * Failover : for disaster recovery scenarios for example. If health check fails, route to secondary
    * Latecy based : route to least latency, based on traffic between user and region
    * Geolocation : route based on user location. Have default location
    * Multi value answer : when routing traffic to multiple resources. NOT a substitute to ELB
    * Geoproximity
        * Based on geographic location of users and resources
        * Bias : shift traffic to specific resource
        * Advanced Route 53 traffic flow
        * Exemple : on coupe une zone géographique, le bias met la découpe plus ou moins proche d'un des bords, les users sont routés en fonction de leur placement par rapport à la découpe
        * Plus le bias est elevé, plus il y aura de traffic vers la resource => plus la découpe est loin
* Health check :
    * On http
    * Only for public resources
    * Integrated into cloud watch
    * Type
        * Endpoint
            * Need return status : 2xx or 3xx
            * Can be based on text and check 5120 first bytes
            * Health checker must be allowed to go in
        * Calculated
            * Combine result of multiple health check
            * Can use OR, AND et NOT
            * Up to 256 children health check
            * Specify how many need to pass to pass
    * For private hosted zone
        * Health checkers are outside VPC
        * Have to create a cloudwatch metric and a cloudwatch alarm then check cloudwatch
* Traffic flow
    * Tool to create complex flow
* Domain Registrar vs DNS Service
    * A registrar provides a DNS service to manage DNS records
    * Possible to buy a domain somewhere and manage it in AWS. Add AWS nameserver to registrar
