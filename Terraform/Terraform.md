# Terraform

Créé et distribué par Hashicorp

Installation : https://developer.hashicorp.com/terraform/downloads

## Notes

* Modules are used to reuse Terraform code
* Terraform chercher les fichiers .tf dans le dossier courant. Du coup, attention aux resources déclarées dans des dossiers différents
* Attention les noms utilisés pour les resources et autres sont unique par compte AWS. Si un même nom est utilisé dans plusieurs scripts une erreur sera lancée (les paramètres 'name =')
* Pour les chemins relatifs, des helpers existent
    * path.module: chemin du filesystem du module où l'expression est définie
    * path.root: chemin du filesystem du root module
    * path.cwd: chemin du filesystem du dossier courant
* Pour rajouter un/des tags communs sur toutes les ressources, utiliser l'inline block default_tags (sauf pour les resources n'acceptant pas les tags et aws_autoscaling_group qui ne prend pas le default_tag)
```
provider "aws" {
    region = "eu-west-3"

    # Tags to apply to all AWS resources by default
    default_tags {
        tags = {
            Owner = "team-foo"
            ManagedBy = "Terraform"
        }
    }
}
```
* Pour utiliser des strings multilines, il est possible d'utiliser EOF
```
output "for_directive_index_if" {
    value = <<EOF
    %{ for i, name in var.names }
        ${ name }%{ if i < length(var.names) - 1 }, %{ endif }
    %{ endfor }
    EOF
}
```
* Utiliser le signe `~`pour trim avant et après le signe. Example
```
output "for_directive_index_if" {
    value = <<EOF
    %{~ for i, name in var.names ~}
        ${ name }%{ if i < length(var.names) - 1 }, %{ endif }
    %{~ endfor ~}
    EOF
}
```