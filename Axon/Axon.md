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


### Architecture CQRS

### Architecture Event sourcing


## Concepts Axon

### Objets : Aggregate, Entities, Command, Event

### Messaging

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