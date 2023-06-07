# Axon

## Introduction

Axon Framework est un framework open source permettant de facilier la mise en place de l'architecture CQRS Event sourcing.

Pour le stockage des événements, AxonIQ propose sa solution Axon Server mais il est également possible d'utiliser n'importe quelle autre base de données.


## Architectures et conception

### Domain Driven Design
Le Domain Driven Design a été mis sur papier par Eric Evans dans son "Blue Book". Parmis tous les principes listés, deux sont primordiaux à retenir :
* Ubiquitous language : il est capital de définir une language commun pour faciliter l'ensemble de la chaîne de production d'un logiciel. Le language commun le plus naturel à adopter est le language métier. Il faut donc que l'ensemble des concepts métier nécessaire se retrouvent dans les logiciels.
* Bounded Context : un logiciel sera plus évolutif et plus maléable si son découpage respecte des frontières qui sont définies par les domaines métier. Qu'il s'agisse de monolithes ou d'architectures microservices, avoir un découpage métier permettra de plus facilement faire évoluer un métier et d'éviter de le dupliquer.

### Architecture event driven
Une architecture event driven repose sur le principe que lorsqu'il se passe quelque chose de structurant au sein d'une application, un événement est émis afin de faire réagir le système. Cela permet notamment de découpler les différents morceaux du systèmes.

Une architecture event driven est souvent associée à un flux de communication via bus de messages, qu'il soit interne (bus Guava par exemple) ou externe (RabbitMQ, Kafka, ...). 

Il ne faut pas confondre les messages qui circulent avec les événements du système. Un message est une structure qui circule au sein d'un bus de message et un événement représente un changement du métier.

### Architecture CQRS
Une application, quelque soit le besoin métier auquelle elle répond, propose des actions d'écriture et des actions de lecture. En moyenne, les opérations de lecture sont plus nombreuses. Egalement, la scalabilité d'une application peut être complexe, qu'il s'agisse de scalabilité horizontale ou scalabilité verticale. Quelques raisons de complexité de scalabilité sont notamment au niveau des opérations d'écriture et de leur concurrence.

L'architecture CQRS, Command Query Responsiblity Segregation/Separation, permet de répondre à ces problématiques. Le principe est de de ségreger les opérations d'écriture des opérations de lecture. Lorsqu'on parle d'opérations, il s'agit d'opérations métier.

Une application respectant l'architecture CQRS permet d'obtenir deux déployables indépendants, un pour l'écriture, un pour la lecture. Chacun ayant son modèle de donnée et son stockage bien distinct. Cela permet de pouvoir scaler les deux morceaux de manière indépendante, sachant que du sharding pour scaler l'écriture intéressant et de la réplication est intéressant pour de la lecture.

### Architecture Event sourcing
Dans certains cas, il y a un besoin fort de conserver un historique de tout ce qu'il se passe au sein d'un système. En partant d'une architecture event driven qui émet des événements métier, il est intéressant de les stocker afin d'avoir un historique très métier.

A partir du moment où l'ensemble des événements d'un système sont stockés, il devient possible de reconstruire entièrement les aggregats (les objets métiers) à partir des événements. On parle alors de système en event sourcing.

## Concepts Axon

### Présentation
Axon framework est un framework CQRS Event sourcing. Il impose une construction d'application respectant ces deux architectures en plus de certains concepts propres à Axon.

Il n'est cependant pas obligatoire d'avoir deux déployables distincts pour la partie lecture et écriture bien que cela soit fortement conseillé.

### Objets : Aggregate, Entities, Command, Event & Value Objects

#### Aggregate
Un aggregat est un objet métier ayant son cycle de vie propre. Un aggregat est alors identifiable uniquement via son identifiant qui doit être unique au sein d'un système. On peut également parler d'aggregat racine (aggregate root) qui est la racine des objets métier.

Etant donné que les aggregats sont identifiés via leur identifiant, il s'agit de class. Avec Kotlin par exemple, il faut les déclarer en `class` et non `data class`

Axon préconise de n'avoir qu'un seul aggregat mais il est possible d'en avoir plusieurs. Cependant, la synchronisation des aggregats doit se faire via des mécanismes comme les saga.

La déclaration d'un aggregat Axon se fait via l'annotation `@Aggregate`.

#### Entities
Un aggregat peut être consistutié d'objets ayant également leur propre cycle de vie. On parle alors d'entité.

La déclaration des entitiés Axon se fait via les annotations `@AggregateMember` mis au niveau de l'aggregat et `@EntityId` mis au niveau de l'identifiant de l'entité.

### Value objects
Les value objects sont des structures de données n'ayant pas de cycle de vie propre mais aggrémente les aggregats d'information. Pour identifier un value object, il faut faire un strict equal sur l'ensemble des champs.

### Command
Une command est un structure de données demandant un changement d'état du système.

Les commandes ne doivent demander qu'un seul changement d'état afin de ne pas se retrouver avec des commandes "fourre-tout" qui peuvent impacter trop d'éléments.

Les commandes respectent le pattern `<VerbeALinfinitif><NomMetier>Command`.

Les commandes doivent obligatoirement cibler les aggregats.

L'aggregat est ciblé via l'annotation `@TargetAggregateIdentifier`.

### Event
Un événement est la représentation d'un changement passé au sein de l'aggregat. Les événements doivent donc contenir l'identifiant de l'aggregat ainsi que les informations strictement nécessaire pour identifier le changement.

Il s'agit de value objects et respectent la convention `<NomMetier><VerbeAuPassé>Event`.


### Messaging
Les flux d'éxecution au sein d'Axon sont régis par des messages et transitent via différents bus. Les objets qui y circulent sont notamment les commands et les events. Ceux-ci sont wrappés dans des objets qui sont respectivement `CommandMessage` et `EventMessage`. Si un événement est envoyé depuis un aggregat, il sera wrappé dans un `DomainEventMessage`.

Les commands et events deviennent alors les payload des messages, ceux-ci comportant également des méta données de type clé/valeur afin de rajouter des informations comme des correlationIds, traceIds ou autre données custom.

#### Command Gateway

#### Event bus
* Tracking/subscribing

### Exception management

### Saga/Process

### Replay

### Snapshot

### Testing

### Configuration


* CQRS l'architecture (idem que dans la partie architecture), DDD, Event driven, Event sourcing (à priori toutes les applications ont besoin de tracing et d'audit)
    * Concepts
        * Aggregate & Entities
        * Command
        * Event
        * Messaging
        * Exception (Axon utilise a fond les exceptions pour arrêter et rollback les transactions)
    * Axon Messaging
        * Command gateway
        * Event bus
    * Configuration
        * Notes de configuration
    * Utilisation
        * Command handler
        * ...
        * Validations (Bean validation with Spring, axon command interceptor, dans la query avec les axon exceptionhandler & ListenerInvocationErrorHandler pour rollback au niveau du catching des events)
        * Saga : synchronisation entre les aggregats
        * Replay
        * Snapshot
        * Testing (unit, integration, jgiven/acceptance) 
            * https://discuss.axoniq.io/t/starting-axon-server-using-testcontainers-from-a-springboottest-port-conflict/2166
            * https://androidexample365.com/running-axon-server-in-testcontainers-tests/
            * https://github.com/holixon/axon-server-testcontainer