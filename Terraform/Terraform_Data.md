# Les sources de data

## Les variables
Exemple de variable
```
variable "db_username" {
  description = "Username of database"
  type        = string
  sensitive   = true
}
```
* Pour utiliser
```
var.<NAME>
```
* Il est possible de définir une default value
* Si pas de défault value, il faut passer la valeur à l'entrée du script
* Ce sont des inputs

## Outputs
* Permet d'exporter des variables de scripts et de les réutiliser ailleurs
* Permet d'afficher dans la console
* Example
```
output "port" {
  value = aws_db_instance.sqldb.port
  description = "The port the database is listening on"
}
```

## Locals
* Il s'agit de variables locales
```
locals {
  VARS...
}
```
* Pour utiliser
```
local.<NAME>
```

## Data sources
* Les data sources permettent de récupérer des informations d'AWS et d'appliquer des opérations dessus (filtrage, paramétrage, ...)
```
data "aws_vpc" "default_vpc" {
  default = true
}

data "aws_subnets" "subnets_in_default_vpc" {
  filter {
    name   = "vpc-id"
    values = [data.aws_vpc.default_vpc.id]
  }
}
```

## Remote data sources
* Cela permet de lire les informations d'un fichier .tfstate stocké quelque part afin d'en récupérer les informations. Pour exemple, afin de découpler la création de base de données de serveurs applicatifs (quand on détruit les serveurs, on ne veut pas forcément détruire les bases de données), on peut utiliser un remote data source
* Exemple
```
# Définition du remote data source
data "terraform_remote_state" "dbcredentials" {
  backend = "s3"

  config = {
    buckuet = "terraform-up-and-running-state"
    key = "stage-data-stores/mysql/terraform.tfstate"
    region = "eu-west-3"
   }
}

resource "aws_launch_configuration" "aninstance" {
  user_data = templatefile("user-data.sh", {
    # Utilisation du remote data source
    db_address  = data.terraform_remote_state.db_instance.outputs.address
    db_port     = data.terraform_remote_state.db_instance.outputs.port
  })
}
```

## Environment variables
* Pour passer des variables d'environnement à un script Terraform, il faut prefix le nom de la variable par TF_VAR_<nom_de_la_variable_terraform>

## Template string
* Il est possible d'utiliser "${...}

## External data source
* Allows an external command that implements a specifitc protocol to act as a data source
* Pass data from Terraform to external program with query argument (received as JSON from stdin)
* Pass external program data to Terraform in JSON to stdout
* Example
```
data "external" "echo" {
  program = ["bash", "-c", "cat /dev/stdin"]

  query = {
    foo = "bar"
  }
}

output "echo" {
  value = data.external.echo.result
}

output "echo_foo" {
  value = data.external.echo.result.foo
}
```