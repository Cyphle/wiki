# Software Architectures

Le point important à noter est qu'il n'existe pas d'architecture qui résoudra tous les problèmes. Chaque architecture répond à des besoins mais apportent également sa complexité. Egalement, les architectures ne sont pas exclusives entre elles.

## CRUD
L'architecture CRUD est assez classique et utile pour les applications qui n'ont pas particulièrement de métier. Il s'agit de séparer les flux d'éxecution en fonction de l'action Create, Read, Update et Delete.

## Hexagonal
L'architecture Hexagonale permet de ségregger la partie métier (le domaine) de l'infrastructure. Cela permet de découpler le métier de tout ce qui est side effect comme l'accès en base de données, l'exposition de endpoints HTTP, ...

Un avantage est par exemple de permettre l'exploitation de données via différentes méthodes (HTTP, envoi dans un bus de message, ...) sans avoir à réécrire le code métier mais uniquement des adapteurs.

Cette architecture repose sur le principe D de Solid : Dependency inversion principle.

Elle repose sur des interfaces et des mappings pour passer d'un monde à l'autre.

## Command Query Responsibility Segregation (CQRS)
Le principe est de séparer tous les flux concernant l'écriture de tous les flux concernant la lecture.

Cela permet de créer des déployables différents pour l'écriture et pour la lecture.

La lecture peut ainsi avoir sa propre base de données et son propre modèle de données. Il peut également y avoir plusieurs modèles de lecture, permettant d'optimiser celle-ci.

Le point important est de rassembler la logique métier entièrement dans l'écriture qui doit contenir la partie domaine.

Cette architecture permet de bien réfléchir à la conception métier car les commands, events et query doivent représenter le métier afin d'être stables dans leur structure.

## Event sourcing
Le principe est d'avoir une architecture qui émet des événements métier à chaque command au sein de l'application. Ces événements sont sauvegardés au sein d'un event store et pour reconstituer les objets, il faut relire les événements dans l'ordre. Cela s'appelle la réhydratation des aggregats. Le stockage ne définit plus qu'une seule table qui est la table des événements.

Typiquement, les événements sont sérialisés au sein de la base de données et sont rajoutés : la date de l'événement (l'ordre est important), le user, la version, le type de l'événement ainsi que l'identifiant de l'aggregate root.

L'event store est immutable. Si une erreur survient, il faut injecter des événements de compensation.

Afin de récupérer les aggregate root, il faut soit récupérer par ID, soit tout récupérer.

## Event Driven
L'architecture event driven permet de faire réagir une application à des événements métiers. Il s'agit de faire circuler des informations métier au lieu de pures structures de données.

## Lambda Core
L'architecture lambda core est l'équivalent de l'architecture hexagonale mais en fonctionnel.