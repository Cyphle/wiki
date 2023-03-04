# Providers

* Providers are plugins that bring necessary language to interact with a cloud provider

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
