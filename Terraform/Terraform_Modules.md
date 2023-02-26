# Modules Terraform

* Permet de mettre le code quelque part et le réutiliser
* Base: tous les fichiers dans un dossier forment un module
* Lancer apply dans un module le marque comme root
* Les provider (comme aws) ne doivent être configurés quand dans le module root
* A chaque modification ou ajout de module, il faut relancer la commande init
* Les modules peuvent être tirés de fichiers locaux, de Git, ou autre

## Utilisation

* Syntaxe de l'utilisation d'un module
```
module "<Name>" {
    source "<Source>"

    [Config...]
}
```
* Name: identifiant d'un module
* Source: chemin vers le module
* Config: arguments

## Inputs

* Permet de recevoir des paramètres d'entrées aux modules comme les noms des resources etc

## Outputs

* Permet de retourner des valeurs pour être utilisée dans les utilisateurs des modules
* Pour utiliser
```
module.<MODULE_NAME>.<OUTPUT_NAME>
```

## Versioning

* Préférer utiliser du versioning pour l'utilisation des modules car si un changement apparait alors toutes les infras seront impactées
* Dans l'import d'un module spécifier l'url du repository ainsi que la version

## Authentication

* Terraform will automatically use SSH key if some modules are in Github