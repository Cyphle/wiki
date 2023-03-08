# Terraform Provisioners
* Execute scripts either on the local machine or a remote machine when running Terraform
* Examples
    * `local-exec`: execute script on local machine
    * `remote-exec`: execute script on a remote resource
    * `file`: copy files to remote resource

## Local exec
```
resource "aws_instance" "example" {
    ami = data.aws_ami.ubuntu.id
    instance_type = "t2.micro"

    provisioner "local-exec" {
        command = "echo \"Hello, World from $(unae -smp)\""
    }
}
```

## Remote exec
* Pour faire cela, le client Terraform doit
    * Savoir communiquer avec l'instance distante (par exemple EC2)
    * S'authentifier sur l'instance en SSH ou WinRM
```
resource "aws_security_group" "instance" {
    ingress {
        from_port = 22
        to_port = 22
        protocol = "tcp"
        cidr_blocks = ["0.0.0.0/0"]
    }
}

resource "tls_private_key" "example" {
    algorithm = "RSA"
    rsa_bits = 4096
}

resource "aws_key_pair" "generated_key" {
    public_key = tls_private_key.example.public_key_openssh
}

data "aws_ami" "ubuntu" {
    most_recent = true
    owners = ["099720109477"]

    filter {
        name = "name"
        values = ["ubuntu/images/hvm-ssd/ubuntu-focal-20.04-amd64-server-*"]
    }
}

resource "aws_instance" "example" {
    ami = data.aws_ami.ubuntu.id
    instance_type = "t2.micro"
    vpc_security_group_ids = [aws_security_group.instance.id]
    key_name = aws_key_pair.generated_key.key_name

    provisioner "remote-exec" {
        inline = ["echo \"Hello, World from $(unae -smp)\""]
    }

    connection {
        type = "ssh"
        host = self.public_ip
        user = "ubuntu"
        private_key = tls_private_key.example.private_key_pem
    }
}
```
* Les commandes sont lancées au apply et à la création initial de la ressource
* Si dans le provsioner, il est spécifié `when = destroy`, les commandes seront lancées au destroy juste avant la suppression de la ressource
* D'autres paramètres `on_failure`, `continue`, `abort`

# Terraform provisioners with null_resource
* Comme les provisioners mais hors des ressources
* Example
```
resource "null_resource" "example" {
    # Use UUID to force this null_resource to be recreated on every call to 'terraform apply'
    triggers {
        uuid = uuid()
    }
    provisioner "local-exec" {
        command = "echo \"Hello, World from $(unae -smp)\""
    }
}
```
* Si une valeur de la null_resource est changée, elle est relancée. 
* Les triggers des null_resources sont une map