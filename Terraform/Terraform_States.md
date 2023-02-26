# State

* Les fichiers .tfstate conservent l'état courant des infras gérées par Terraform. 
* Afin de travailler en équipe et qu'il n'y ait pas de conflits avec les states, il faut paramétrer un backend Terraform partagé où sont stockés les états.

# Backend
* Un backend permet de stocker les fichiers .tfstate contenant les states dans une source déportée comme S3
* La configuration des backends ne permet pas l'utilisation de variables. Solution
    * Utiliser des partial configurations
    * Ne pas mettre certains paramètres et les passer via -backend-config command line arguments when calling terraform init
    * Exemple
        - backend.hcl
        ```
        bucket = "s3_bucket_name"
        region = "aws_region"
        dynamodb_table = "table_name"
        encrypt = true
        ```
        - Then use terraform init -backend-config=backend.hcl
    * Une autre solution est d'utiliser Terragrunt

# Remote state
* Permet de récupérer les state files d'autres Terraform
```
data "terraform_remote_state" "dbcredentials" {
  backend = "s3"

  config = {
    buckuet = <bucket_name>
    key = "stage-data-stores/mysql/terraform.tfstate"
    region = "eu-west-3"
   }
}
```
