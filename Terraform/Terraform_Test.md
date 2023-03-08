# Test avec Terraform

* Aller voir le repository Terratest pour des exemples de tests et des outils de test

## Module self validation
### Validations
* Validation bloc sur les inputs. Attention cela ne permet que de vérifier l'input courant et il n'est pas possible de faire référence à d'autre élément.
```
variable "instance_type" {
    description = "The type of EC2 Instances to run (e.g. t2.micro)"
    type = string

    validation {
        condition = conttains(["t2.micro", "t3.micro"], var.instance_type)
        error_message = "Only free tier is allowed: t2.micro | t3.micro."
    }

    validation {
        condition = var.min_size <= 10
        error_message = "ASGs must have 10 or fewer instances to keep costs down."
    }
}
```

### Preconditions & postconditions
* Preconditions are for catching errors before running apply
```
data "aws_ec2_instance_type" "instance" {
    instance_type = var.instance_type
}

resource "aws_launch_configuration" "example" {
    image_id = var.ami
    instance_type = var.instance_type
    security_groups = [aws_security_group.instance.id]
    user_data = var.user_data

    # Required when using a launch configuration with an auto scaling group
    lifecycle {
        create_before_destroy = true
        precondition {
            condition = data.aws_ec2_instance_type_instance_free_tier_eligible
            error_message = "${var.instance_type} is not part of the AWS Free tier !"
        }
    }
}
```
* Postconditions are for catching errors after running apply. Par exemple pour check qu'un ASG a été deployé cross AZ
```
resource "aws_autoscaling_group" "example" {
    name = var.cluster_name
    launch_configuration = aws_launch_configuration.example.name
    vpc_zone_identifier = var.subnet_ids

    lifecycle {
        postcondition {
            # Keyword self can only be used in postconditions to refer to output
            condition = length(self.availability_zones) > 1
            error_message = "You must use more than one AZ for high availability!"
        }
    }
}
```

### When to use validations, preconditions & postconditions
* Validation for basic input sanitization
* Precondition for basic asumptions
* Postcondition for basic guarantees
* Automated tests for other advanced checks