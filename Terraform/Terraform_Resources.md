# Resources Terraform

* Il s'agit des éléments d'infrastructure à générer (EC2, ELB, ...)

## Provider

* Les ressources étant dépendantes du cloud provider, il faut le définir en début de script dans le root module (celui qui est lancé)
```
provider "aws" {
  region = "eu-west-3"
}
```

## Définition d'une ressource

```
resource "<RESOURCE_DEFINITION>" "<NAME>" {
  [CONFIG...]
}
```

Exemple
```
resource "aws_instance" "MyFirstInstance" {
  ami           = "ami-0ca5ef73451e16dc1"
  instance_type = "t2.micro"
  # aws_security_group.instance.id reference l'id du security group de l'instance
  vpc_security_group_ids = [aws_security_group.instance.id]

  user_data = <<-EOF
        #!/bin/bash
        echo "Hello, World" > index.html
        nohup busybox httpd -f -p ${var.server_port} &
        EOF

  user_data_replace_on_change = true

  tags = {
    Name = "terraform_example"
  }
}
```

## Inline blocks

* Argument défini à l'intérieur d'une ressource
```
resource "xxx" "yyy" {
    <NAME> {
        [CONFIG...]
    }
}
```
* Dans les modules, préférer utiliser des resources séparées que des inline blocks sinon cela créer des conflits si on veut override et le template ne compilera pas

## Recherche d'AMI
* Les ressources AWS permettent de rechercher une AMI afin d'obtenir son ID
```
data "aws_ami" "ubuntu_region_1" {
  provider = aws.region_1

  most_recent = true
  downers = ["099720109477"]

  filter {
    name = "name"
    values = ["ubuntu/images/hvm-ssd/ubuntu-focal-20.04-amd64-server-*"]
  }
}
```