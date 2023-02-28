# Secrets

## Personnal access
* Avec AWS, il est possible d'utiliser `aws-vault` pour stocker des access key et les communiquer directement à AWS CLI ou Terraform
```
aws-vault add dev

aws-vault exec <PROFILE> -- <COMMAND>

aws-vaule exec dev -- terraform apply
```
exec command va appeler AWS STS pour générer un token temporaire et l'exposer en variable d'environnement.

## Machine access
* Authentication machine to machine par exemple un CI/CD qui doit appeler l'API AWS.

### Example: CircleCI
* Stocker les credentials AWS dans l'IHM CircleCI et dans un context et utiliser ce context dans la définition du pipeline
```
workflows:
    deploy:
        job:
            - terraform apply
        filters:
            branches:
                only:
                    - main
        context:
            - example-context
```

### Example: EC2 Instance running Jenkins
* Solution recommandée : donner un rôle IAM à l'EC2. Il faut que le rôle IAM possède une policy pour assumer le rôle en plus des autres policies.
```
data "aws_iam_policy_document" "assume_role" {
    statement {
        effect = "Allow"
        actions = ["sts:AssumeRole"]

        principals {
            type = "Service"
            identifiers = ["ec2.amazonaws.com"]
        }
    }
}

resource "aws_iam_role" "instance" {
    name_prefix = var.name
    assume_role_policy = data.aws_iam_policy_document.assume_role.json
}

data "aws_iam_policy_document" "ec2_admin_permissions" {
    statement {
        effect = "Allow"
        actions = ["ec2:*"]
        resources = ["*"]
    }
}

resource "aws_iam_role_policy" "example" {
    role = aws_iam_role.instance.id
    policy = data.aws_iam_policy_document.ec2_admin_permissions.json
}

resource "aws_iam_instance_profile" "instance" {
    role = aws_iam_role.instance.name
}

resource "aws_instance" "example" {
    ami = "ami-xxx"
    instance_type = "t2.micro"

    # Attach the instance profile
    iam_instance_profile = aws_iam_instance_profile.instance;name
}
```
Les outils comme Terraform qui utilisent le SDK AWS savent utiliser les meta data de l'instance

### Example: GitHub Actions
* Il est possible d'utiliser OIDC pour ouvrir une connexion avec le cloud provider.
```
# GitHub Action
name: Terraform Apply
permissions:
    id-token: write
on:
    push:
        branches:
            - 'main'
jobs:
    TerraformApply:
        runs-on: ubuntu-latest
        steps:
            - uses: actions/checkout@v2
            - uses: aws-actions/configure-aws-credentials@v1
              with:
                # Specify the IAM role to assume here
                role-to-assume: arn:aws:iam:1234:role/example-role
                aws-region: us-east-2
            - uses: hashicorp/setup-terraform@v1
              with:
                    terraform_version: 1.1.0
                    terraform_wrapper: false
              run: |
                terraform init
                terraform apply -auto-approve

# Terraform to create an IAM OIDC identity provider that trusts GitHub
resource "aws_iam_openid_connect_provider" "github_actions" {
    url = "https://token.actions.githubusercontent.com"
    client_id_list = ["sts.amazonaws.com"]
    thumbprint_list = [
        data.tls_certificate.github.certificates[0].sha1_fingerprint
    ]
}

# Fetch GitHub's OIDC thumbprint
data "tls_certificate" "github" {
    url = "https://token.actions.githubusercontent.com"
}

data "aws_iam_policy_document" "assume_role_policy" {
    statement {
        actions = [ "sts:AssumeRoleWithWebIdentity"]
        effect = "Allow"

        principals {
            identifiers = [aws_iam_openid_connect_provider.github_actions.arn]
            type = "Federated"
        }

        condition {
            test = "StringEquals"
            variable = "token.actions.githubusercontent.com:sub"
            # The repos and branches defined in var.allowd_repos_branches will be able to assume this IAM role
            values = [
                for a in var.allowed_repos.branches:
                    "repos:${a["org"]}/${a["repo"]}:ref:refs/heads/$(a["branch"]}"
            ]
        }
    }
}

variable "allowed_repos_branches" {
    description = "GitHub repos/branches allowed to assume the IAM role"
    type = list(object({
        org = string
        repo = string
        branche = string
    }))
    # Example:
    # allowed_repos_branches = [
    #     {
    #         org = "brikis98"
    #         repo = "terraform-up-and-running-code"
    #         branch = "main
    #     }
    # ]
}
```