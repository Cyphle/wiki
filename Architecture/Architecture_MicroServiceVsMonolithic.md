# Micro services vs Monolithic

## What is a microservice architecture?
Une architecture microservice est une architecture où plusieurs applications, au sens de plusieurs déployables, travaillent ensemble pour constituer un tout. L'intérêt est d'abord de pouvoir développer des applications qui ont peu de couplable mais beaucoup de cohésion.

## What is a monolithic architecture?
Une architecture monolithic est une architecture où plusieurs fonctions d'un même logiciel sont développées au sein d'une même base code source, cela produira un unique déployable.

## Pros & Cons
Avant de lister quelques avantages et inconvénients de chacune des architecture, il est important de noter qu'il n'y a pas une architecture meilleure que l'autre. Chacune doit êre utilisée selon le besoin. Comme tout concept en informatique, il n'y pas d'outil ou de solution magique qui répondra à tous les problèmes. Il faut savoir considérer quoi utiliser dans quel cas.

### Monolithic
Pros
* Un seul déployable
* Facilité de déploiement
* Pas de complexité de responsabilité en tant que service
Cons
* Forte dépendance entre les différents domaines métiers
* Mélange de toutes les données dans une même base de données
* Single point of failure
* Dépendances entre les équipes

### Microservices
Pros
* Multiple points of failures
* Indépendance d'évolution
* Possibilité de scale différemment
Cons
* Complexité de déploiement
* Complexité de synchronisation
* Complexité de définir les responsabilités

## How to design microservices
Le premier facteur de définition de microservices est la responsabilité métier. Il faut déssiner des bounded cnotexts représentant chacun un domaine métier distinct. Par exemple, le service expédition et le service facturation d'un marchant. Des outils existent pour définir les responsabilités métier tels que le DDD (Domain Driven Deisgn) et l'Event Storming.