# What is IAM & STS ?

* IAM est le gestionnaire d'identité d'AWS.
* STS is Security Token Service

# IAM
* Il permet de gérer
	* Users
	* Groupes
	* Security policies
	* Roles (users et applications)
* Ne pas utiliser le root account. Préférer des comptes users
* Privilégier les habilitations sur les groupes
* Roles are used for applications to use other applications
* Not recommanded to use inline policies
* To test policies : 
	* use AWS policy simulator
	* Use the CLI : --dry-run on some commands (ex : aws ec2 run-instances --dry-run --image-id ami-ID --instance-type t2.micro) to decode authorization errors use STS aws decode or add DecodeAuthorizationMessages permission in STS or sts decode-authorization-message command
* Authorization model, evaluation of policies
	1. If htere's an explicit DENy, decision deny
	2. If there's an ALLOw, decision ALLOW
	3. Else deny
* IAM policies & S3 bucket policies
	* IAM policies are attached to users, roles, groups
	* S3 bucket poclicies are attached to buckets
	* When evaluating if user can perform something on a bucket, it is union of IAM policies and S3 bucket policies
	* Example 1: IAM role attached to EC2 authorizes RW + nothing on bucket => authorize RW
	* Example 2: IAM role attached to EC2 authorizes RW + S3 policy explicit deny to the IAM role => deny
	* Example 3: IAM role attached to EC2 no bucket permissions + S3 policy allow RW => allow
	* Example 4: IAM role attached to EC2 explicit deny + S3 policy explicit RW => deny
* Dynamic Policies with IAM
	* Use case: how to assign each user a /home/<user> folder in S3 bucket?
		* Option 1: Create IAM policy allowing each user to access his /home 
		* Option 2: Use dynamic policy by using policy variable `${variable}` (ex: ${aws:username})
* Inline vs Managed policies
	* AWS Managed policy: created & maintained by AWS
	* Customer manager policy: created by user
	* Inline policy: string one-to-one between policy and principal. Assigned directly to principal (AWS user)
* Granting a user permissions to pass a role to an AWS service
	* To configure many AWS services, must pass an IAM role to the service
	* Need IAM permission iam:PassRole
	* Often comes with iam:GetRole to view role being passed
	* Roles can only be passed to what their trust allows
		* A trust policy for the role that allows the service to assume the role
		* Target service needs to assume the role to use the source service
* Assuming a role involves using a set of temporary security credentials that you can use to access AWS resources that you might not normally have access to. These temporary credentials consist of an access key ID, a secret access key, and a security token. Typically, you assume a role within your account or for cross-account access.

# STS
* STS : Security Token Service
	* Allow to generate temporary tokens to access resources
* Allows to grant limited and temporary access to AWS resources (up to 1 hour)
* AssumeRole: assume role within your account or cross account
	* Define an IAM role within an account
	* Define which principals can access IAM role
	* Use STS to retrieve credentials and impersonate IAM role
	* Temporary credentials can be valid between 15min to 1hou
* AssumeRoleWithSAML: return credentials for users logged with SAML
* AssumeRoleWithWebIdentity: return creds for users logged wit an IDP like Cognito Identity Pools
* GetSessionToken: for MFA
	* Need appropriate IAM policy using IAM conditions
	* aws:MultiFactorAuthPresent:true in policies is needed
	* Has access id, secret key, session token, expiration date
* GetFederationToken: obtain temporary creds for federated user
* GetCallerIdentity: return details about IAM user or role used in API call
* DecodeAuthorizationMessage: decode error message when an AWS API is denied
