# What is CloudFront

Il s'agit d'un CDN utilisé principalement pour optimiser les temps de réponse. Il s'agit d'une solution globale.

# Notes
* Is a CDN
* Content is cached at the edge
* 216 points in the world
* DDoS protectin with shield 
* Origins of cloudfront content
    * S3 : security with OAC (origin access control)
        * Cloudfront can be used as an ingress to upload files to s3
    * Custom origin
        * Applicaiton load balancer
        * Ec2
        * S3 website
        * Any http backend
* Can setup TTL (0s to 1 year)
* Cache based on
    * Headers
    * Session cookies
    * Query string parameters
    * Can be partially invalidated
    * Can differentiate static and dynamic content to maximize cache
* Each edge has its cache
* For EC2 as origin, security group must be public and allows all edge location ips. No private connexion between EC2 and CloudFront
* For ALB, it must be public
* Geo restriction
    * Allowlist or blocklist
* CloudFront can be private. To allow some users need to use signed url or signed cookie. Attach a policy
    * Signed URL for individual files
        * Can filter by ip, path, date, expiration
        * Account wide key-pair (S3 presigned url have to choose the issuer of presigned url)
        * Signers
            * Trusted key group (recommanded)
                * Rotate keys (and iam for api security)
            * An aws account that contains a cloudfront key pair
                * Need to manage keys using root account
    * Signed cookie for multiple files
    * Attention pricing varies depending on regions
        * Possible to reduce number of edge locations
    * Price classes
        * Class All : all regions
        * Class 200 : excluse the most expensive regions
        * Class 100 : only the cheapests
    * Multi origin
        * Route to different origin based on content type
            * Based on path pattern
    * Origin groups
        * Increase HA
        * One primary origin and one secondary
    * Field level encryption
        * Protect sensitive protection, additional protection
        * Edge location encrypt content
        * User must have key to decrypt (asymetric key)
