# Commandes Terraform

## Terraform init
``` terraform init ```
* Initialise le projet avec les informations du provider utilisé dans les scripts
* Paramètres
    * -backend-config=<file>: permet de passer une configuration partielle de backend
* Configure les backends, installe les providers et télécharge les modules

## Terraform plan
``` terraform plan ```
* Lance un dry-run des scripts
* Paramètres
    * -out=example.plan pour stocker la diff

## Terraform apply
``` terraform apply ```
* Applique les scripts
* Paramètres
    * <nom_de_fichier_issue_de_plan> pour appliquer strictement les diff calculées par plan

## Terraform graph
``` terraform graph ```
* Display dependency graph of deployment

## Terraform destroy
``` terraform destroy ```
* Clean up everything in the script

## Terraform workspace
``` terraform workspace ```
* Permet de basculer entre workspaces

``` terraform workspace show ```
* Permet de voir le workspace courant

``` terraform workspace new <workspace_name> ```
* Créer un workspace

``` terraform workspace list ```
* Liste les workspaces

``` terraform workspace select <workspace_name> ```
* Bascule sur le workspace workspace_name

## Terraform console
``` terraform console ```
* Permet de tester des fonction
* Récupérer l'état des infra
* En lecture seule

## Terraform import
``` terraform import <address> <resource_id> ````
* Import les infos d'une infrastructure déjà existante dans le fichier state
* address: `<PROVIDER>_<TYPE>.<NAME>` (ex: aws_iam_user.existing_user)
* resource_id (ex: foo.bar)
* Alternative tools: terraformer et terracognita

## Move state file
``` terraform state mv <ORIGINAL_REFERENCE> <NEW_REFERENCE> ```
* Déplace un state en modifiant une ressource
* Cela permet de par exemple renommer une resource sans avoir à redeployer.
    * Example `terraform state mv aws_security_group.instance aws_security_group.cluster_instance`
