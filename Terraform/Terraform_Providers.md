# Providers

* Providers are plugins that bring necessary language to interact with a cloud provider

## Notes
* Il faut utiliser les alias de provider avec parcimonie car cela peut coupler le déploiement des région. Il faut préférer déployer les régions de manière indépendante afin de minimiser le risque de tout impacter s'il y a une erreur. De préférence, utiliser les alias lorsque le déploiement de ressource multi région est fortement couplé au reste comme le déploiement de CloudFront et ses certificats TLS qui doivent être déclarés dans la région us-east-1.
* Il ne faut pas définir le provider dans les reusable modules. C'est un antipattern, cela masque une configuration obligatoire en dur.

## Basic
* For basic use of provider
```
provider "aws" {
    region = "eu-west-3"
}
```
* Cela va utiliser par défaut le provider aws dans le repository Hashicorp avec la dernière version

## More control
* For more control
```
terraform {
    required_providers {
        <LOCAL_NAME> = {
            source = "<URL>"
            version = "<VERSION>"
        }
    }
}
```
    * LOCAL_NAME: le nom utilisé dans les scripts comme "aws"
    * URL: l'url où récupérer le plugin comme "registry.terraform.io/hashicorp/aws"
    * VERSION: version du plugin
```
terraform {
    required_providers {
        aws = {
            source = "hashicorp/aws"
            version = "~> 4.0"
        }
    }
}
```
* Il faut ensuite rajouter
```
provider "aws" {
    region = "eu-west-3"
}
```

## Multiple copies of same provider

### Multiple AWS regions
* Il est possible d'ajouter des alias afin de pouvoir spécifier plusieurs régions
```
provider "aws" {
    region = "eu-west-3"
    alias = "region_1"
}

provider "aws" {
    region = "us-west-1"
    alias = "region_2"
}

data "aws_region" "region_1" {
    provider = aws.region_1
}

data "aws_region" "region_2" {
    provider = aws.region_2
}

data "aws_ami" "ubuntu_region_1" {
  provider = aws.region_1

  most_recent = true
  downers = ["099720109477"]

  filter {
    name = "name"
    values = ["ubuntu/images/hvm-ssd/ubuntu-focal-20.04-amd64-server-*"]
  }
}

resource "aws_instance" "region_1" {
    provider = aws.region_1

    ami = data.aws_ami.ubuntu_region_1.id
    instance_type = "t2.micro"
}

resource "aws_instance" "region_2" {
    provider = aws.region_2

    ami = "ami-yyy"
    instance_type = "t2.micro"
}

output "region_1" {
    value = data.aws_region.region_1.name
    description = "The name of the first region"
}

output "region_2" {
    value = data.aws_region.region_2.name
    description = "The name of the second region"
}

output "instnace_region_1_az" {
    value = aws_instance.region_1.availability_zone
    description = "The AZ where the instance in the first region deployed"
}
```

## Modules
* To deploy a module in a specified region
```
# In the module
terraform {
  required_providers {
    aws = {
      source = "hashicorp/aws"
      version = "~> 4.0"
    }
  }
}

# In the one using the module
module "mysql_primary" {
    source = "../../../modules/data-stores/mysql"

    providers = {
        # Set provider with a provider alias. It references the local_name of required_provider of the module
        aws = aws.primary
    }

    [CONFIG...]
}
```

## Multiple accounts
* Il est une bonne pratique de déployer les environnements dans des comptes AWS séparés (idem pour d'autres cloud providers)
* Pour utiliser un autre compte, le mettre dans le provider
```
provider "aws" {
    region = "us-east-2"
    alias = "child"

    assume_role {
      role_arn = "arn:aws:iam:<ACCOUNT_ID>:role/<ROLE_NAME>"
    }
}
```

## Configuration alias
* A peu près la même chose sauf que ce n'est pas un alias de provider mais un alias de required_providers. Il s'agit des alias requis par un module pour référencer un ou plusieurs providers
```
terraform {
    required_providers {
        aws = {
            source = "hashicorp/aws"
            version = "~> 4.0"
            configuration_aliases = [aws.parent, aws.child]
        }
    }
}
```