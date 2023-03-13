# Terragrunt

* Terragrunt comble les manques de Terraform en apportant par exemple la possibilité d'utiliser des modules via fichiers de configuration pour éviter de se répéter
* Il est possible de faire de l'héritage de configuration via
```
include {
    path = find_in_parent_folders()
}
```
* Exemple
```
# In a module add
provider "aws" {
    region = "eu-west-3"
}

# Use of a module in terragrunt.hcl
terraform {
    source = "github.com/<ODNWER>/modules/services/hello-world-app?ref=v0.0.7"
}

include {
    path = find_in_parent_folders()
}

# Read outputs of module
dependency "mysql" {
    config_path = "../../data-stores/mysql"
}

inputs = {
    environment = "stage"
    ami = "ami-xxx"

    min_size = 2
    max_size = 2

    enable_autoscaling = false

    mysql_config = dependency.mysql.outputs
}
```

## Commands
* `terragrunt apply` même chose que terraform apply mais va aussi chercher les fichiers terragrunt.hcl

## Pour les backend
```
# terragrunt.hcl
remote_state {
    backend = "s3"

    generate = {
        path = "backend.tf"
        if_exists = "overwrite"
    }

    config = {
        bucket = "<YOUR_BUCKET>"
        key = "${path_relative_to_include()}/terraform.tfstate"
        region = "us-east-2"
        encrypt = true
        dynamodb_table = "<YOUR_TABLE>"
    }
}
```