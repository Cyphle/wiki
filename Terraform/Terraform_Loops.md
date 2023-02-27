# Terraform Loops

## count
* All resources have the parameter count to tell how many times to create resource
```
variable "user_names" {
  description = "Create IAM useres with these names"
  type        = list(string)
  default     = ["neo", "trinity", "morpheus"]
}

resource "aws_iam_user" "example" {
  count = length(var.user_names)
  name  = var.user_names[count.index]
}
```
* La resource IAM user est maintenant une liste de ressource car créée avec count. Pour y faire référence, il faut accéder à l'index ou * pour toutes
```
<PROVIDER>_<TYPE>.<NAME>[INDEX].ATTRIBUTE
```
* Count parameter peut être utilisé lors d'import de modules
* Ne peut pas être utilisé pour les inline blocks
* Terraform identifie les resources par leur index donc si un élément est supprimé, les resources seront shiftées et la resource supprimée sera la dernière avec chaque élément n+1 remplaçant n avec n l'index du supprimé. Comme pour Terraform les resources sont immutables, elles seront toutes supprimées à partir de n et recréées

## for_each
* Allow to create copies of
  * Resource
```
resource "<PROVIDER>_<TYPE>" "<NAME>" {
  for_each = <COLLECTION>

  [CONFIG...]
}
```
  * Inline block within a resource
```
dynamic "<VAR_NAME>" {
  for_each = <COLLECTION>

  content {
    [CONFIG...]
  }
}
```
  * Module
```
module "users" {
  source = "../../../modules/landing-zone/iam-user"

  for_each = toset(var.user_names)
  user_name = each.value
}
```
* Iterate through set or map (lists are not supported)
* Access each.key and each.value
* Output map with keys == each.key and not list as with count
* Comme on obtient une map, si une entrée est suppriéme au milieu du set, cela ne créé pas de shift des autres contrairement à count

## for
* Syntaxe pour une liste
```
[for <ITEM> in <LIST> : <OUTPUT> [if [CONDITION]]]
```
* Syntaxe pour une map
```
[for <KEY>, <VALUE> in <MAP> : <OUTPUT>]
```
* Il est possible de filtrer sur un critère comme les list comprehension de Python
* Pour output une map
```
{for <ITEM> in <LIST> : <OUTPUT> => <OUTPUT_VALUE>}

{for <KEY>, <VALUE> in <MAP> : <OUTPUT_KEY> => <OUTPUT_VALUE>}
```

## for String Directive
* Une String directive permet d'utiliser des éléments de contrôle dans une chaîne de caractères
```
%{...}
```
* Syntaxe
```
%{for <ITEM> in <COLLECTION>}<BODY>%{endfor}

%{for <INDEX>, <ITEM> in <COLLECTION>}<BODY>%{endfor}
```

## Loops limitations
* Cannot reference outputs in count or for_each but can reference data, variables, lists of resources. Example:
```
resource "random_integer" "num_instances" {
  min = 1
  max = 3
}

resource "aws_instance" "example_3" {
  # This won't work because it cannot determine value before execution
  count = random_integer.num_instances.result
  ami = "ami-xxx"
  instance_type = "t2.micro"
}
```
