# Isolation

* Il faut pouvoir isoler la configuration des différents environnements.

## Workspaces

* Via workspaces: pour des tests rapides et isolées avec la même configuration
* Permet de gérer des états différents par workspace.
* Par défaut, travaille avec le workspace "default"
* Cela va créer des fichiers tfstate différents par workspace

## File layout

* Via file layout: pour de la prod. Séparation forte envtre environnements
