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

## Ressources et Data Sources
* Pour passer des variables
    * Environment variables
    * Encrypted files
    * Secret stores

### Environment variables
* Utiliser des variables
```
variable "db_username" {
    description = "The user for the database"
    type = string
    sensitive = true
}

variable "db_password" {
    description = "The password for the database"
    type = string
    sensitive = true
}
```
* Le `sensitive = true` indique qu'il s'agit de secrets et Terraform ne le printera pas. 
* Pour les utiliser, déclarer les variables
```
export TF_VAR_db_username=...
export TF_VAR_db_password=...
```

### Encrypted files
* Mettre les infos dans un texte, le chiffrer et le stocker quelque part (S3, version control, ...)
* Mettre la clé de chiffrement dans un outil comme KMS ou utiliser les clés Github des développeurs par exemple
* Pour chiffrer et déchiffrer des fichiers de manière "transparente", il est possible d'utiliser des outils comme sops

#### Exemple d'utilisation de KMS en créant une CMK (Customer Managed Key)
1. Création de la clé KMS
```
provider "aws" {
    region = "us-east-2"
}

# Donne des informations sur le user courant
data "aws_caller_identify" "self" {}

# Donne au user courant les droits d'admin sur la CMK
data "aws_iam_policy_document" "cmk_admin_policy" {
    statement {
        effect = "Allow"
        resources = ["*"]
        actions = ["kms:*"]
        principals {
            type = "AWS"
            identifiers = [data.aws_caller_identity.self.arn]
        }
    }
}

# Créer la CMK
resource "aws_kms_key" "cmk" {
    policy = data.aws_iam_policy_document.cmk_admin_policy.json
}

# Alias pour la clé
resource "aws_kms_alias" "cmk" {
    name = "alias/kms-cmk-example"
    target_key_id = aws_kms_key.cmk.id
}
```
2. Craétion du fichier chiffré
```
FILE (db-creds.yml):
username: admin
password: password

BASH TO ECNRYPT (encrypt.sh):
#!/bin/bash

CMK_ID="$1"
AWS_REGION="$2"
INPUT_FILE="$3"
OUTPUT_FILE="$4"

echo "Encrypting contents of $INPUT_FILE using CMK $CMK_ID..."
ciphertext=$(aws kms encrypt \
    --key-id "$CMK_ID" \
    --region "$AWS_REGION" \
    --plaintext "fileb://$INPUT_FILE" \
    --output text \
    --query CiphertextBlob)

echo "Writing result to $OUTPUT_FILE..."
echo "$ciphertext" > "$OUTPUT_FILE"

echo "Done!"

BASH USE:
./encrypt.sh alias/kms-cmk-example us-east-2 db-creds.yml db-creds.yml.encrypted
```
3. Récupérer les secrets dans Terraform
```
data "aws_kms_secrets" "creds" {
    secret {
        name = "db"
        payload = file("${path.module}/db-creds.yml.encrypted")
    }
}

locals {
    db_creds = yamldecode(data.aws_kms_secrets.creds.plaintext["db"])
}

resource "aws_db_instance" "example" {
    identifier_prefix = "terraform-up-and-running"
    engine = "mysql"
    allocated_storage = 10
    instance_class = "db.t2.micro"
    skip_final_snapshot = true
    db_name = var.db_name

    username = local.db_creds.username
    password = local.db_creds.password
}
```

### Secret stores
* Exemples de secret stores : AWS Secrets Manager, Google Secret Manager, Azure Key Vault, Hashicorp Vault
* Pour utiliser AWS Secret Manager dans Terraform
```
# Récupérer un secret enregistré dans Secret Manager
data "aws_secretsmanager_secret_version" "creds" {
    secret_id = "db-creds"
}

locals {
    db_creds = jsondecode(data.aws_secrets_manager_secret_version.creds.secret_string)
}
```

## State files and Plan files

### State files
* ATTENTION : tous les secrets utilisés dans les ressources et data sources Terraform sont stockés en clair dans les state files.
* Pour parer à cette contrainte Terraform
    * Stocker les fichiers state dans un backend qui supporte le chiffrement (comme S3)
    * Mettre une policy stricte sur qui a accès au backend

### Plan files
* Avec la commande plan, il est possible de stocker la diff dans un fichier avec `terraform plan -out=example.plan`
* Il est alors possible d'appliquer strictement la diff avec `terraform apply example.plan`
* Les secrets sont en clairs dans les fichiers plans
* Pour utiliser les fichiers plan
    * Chiffrer les fichiers plan
    * Mettre une policy stricte sur qui a accès à ces fichiers
    