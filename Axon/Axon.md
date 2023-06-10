# Axon

## Introduction

Axon Framework est un framework open source permettant de facilier la mise en place de l'architecture CQRS Event sourcing.

Pour le stockage des événements, AxonIQ propose sa solution Axon Server mais il est également possible d'utiliser n'importe quelle autre base de données.


## Architectures et conception

### Domain Driven Design
------
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

Les modèles de lecture sont appelés des projections.

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

Axon étant en Event Sourcing, les events du domaine sont enregistrés dans un Event store. Cet Event store peut être n'importe quelle base de données ou Axon server.


### Messaging
Les flux d'éxecution au sein d'Axon sont régis par des messages et transitent via différents bus. Les objets qui y circulent sont notamment les commands et les events. Ceux-ci sont wrappés dans des objets qui sont respectivement `CommandMessage` et `EventMessage`. Si un événement est envoyé depuis un aggregat, il sera wrappé dans un `DomainEventMessage`.

Les commands et events deviennent alors les payload des messages, ceux-ci comportant également des méta données de type clé/valeur afin de rajouter des informations comme des correlationIds, traceIds ou autre données custom.


### Message correlation
Les messages peuvent être corrélés entre. Cela est particulièrement utile pour tracer toute une chaîne d'exécution. Il faut placer les id de corrélation dans les méta data des messages.

Pour générer ces id de corrélation, il faut utiliser des `CorrelationDataProvider`. De base, est fourni `MessageOriginProvider` qui insère des `correlationId` et des `traceId`, avec les correlationId référençant le message parent et les traceId référençant le message d'origine.

Afin de créer des custom provider, il faut implémenter l'interface `CorrelationDataProvider`. Il est également possible d'utiliser des providers fournis comme `SimpleCorrelationDataProvider` permettant d'insérer des meta data ou `MultiCorrelationDataProvider` permettant d'utiliser plusieurs providers en même temps.

#### Command utilisation
Les commandes doivent être envoyées dans une command gateway. Par exemple `commandGateway.post(<MaCommand>)`. Il est important de noter que les commandes envoyées doivent être typées, il ne faut pas envoyer de liste en raison du type erasure de Java qui ne permettra pas un routing correct. De plus, le pattern veut que les commandes soient envoyées une à une.

##### Command handling
Les commands sont récupérées par des handlers qui se trouvent dans les aggregats (y compris les commandes de création) via des annotations `@CommandHandler` et la commande en paramètre.

Les commandes ne doivent être traitées que par un seul et unique handler.

##### Command dispatch interception & validation
Il est possible de valider des commandes avant qu'elles arrivent au niveau des handlers. Pour cela, il faut implémenter `MessageDispatchInterceptor` et enregistrer la class.

Il est alors possible d'accéder aux messages de commandes et appliquer des opérations comme ajouter des ID de correlation ou valider les commandes.

Il est également possible d'utiliser les validateurs Hibernates comme `@NotNull` sur les champs des commandes.

Pour valider l'unicité des aggregats, il existe le pattern `set based validation` qui requiert de faire une projection côté commande. Mais ce pattern n'est pas conseillé. Une manière plus élégante est d'utiliser les attributs servant de check d'unicité et créer un hash à partir ce ceux-ci, hash qui va servir d'aggregate identifier.

##### Command handler interception
Parfois, il est nécessaire d'avoir des informations sur l'aggregat pour faire des opérations, comme valider, des commandes. Pour cela, il existe les command handler interceptor qui se trouvent dans les aggregats. Il s'agit de fonction portant l'annotation `@CommandHandlerInterceptor`.

Ces fonctions ont alors accès au `UnitOfWork` contenant le message de commandes. Le deuxième paramètre de ces fonctions est la chaîne d'interceptors (dans le sens responsiblity chain). Une commande non validée peut lancer une exception et une commande validée va procéder à la suite de la chaîne avec `interceptorChain.proceed()`.

#### Event utilisation
Les events doivent être envoyés dans des bus de messages via la méthode `AggregateLifeCycle.apply(<Event>)`. Il s'agit d'une méthode statique afin de ne pas avoir à injecter de dépendance dans les aggregats.

##### Event handling
Les events sont récupérées par des handlers qui se trouvent soit dans les aggregats via l'annotation `@EventSourcingHandler` avec l'event en paramètre pour pouvoir les reconstruire (principe Event Sourcing), soit dans des handlers permettant de construire des projections via l'annotation `@EventHandler` avec l'event en paramètre.

##### Listening mode
Les event handlers ont deux modes de récupération des events :
* Tracking : qui est le mode par défaut. Cela créé un thread par event handler (non event sourcing handler) avec un mode 'pull'
* Subscription : qui n'ouvre qu'un seul thread pour tous les event handlers (non event sourcing handler) appartenant aux class ayant l'annotation `@ProcessingGroup(<NomDuProcessor>)` avec le même nom de processeur. Ce mode de récupération d'events permet notamment d'appliquer des mécanismes de rollback dans le cas où un event handler échoue, tous les autres vont échouer et la transaction jusqu'à la command est rollback. Il n'y aura donc pas d'event enregistré dans l'event store.

Il faut également ajouter une configuration dans `application.yml` pour définir le mode d'écoute, par exemple : `axon.eventhandling.processors.<processorName>.mode: subscribing|tracking`. Les `@ProcessingGroup` ne sont pas forcément lié à des modes d'écoute subscription.


### Exception management
// TODO
* With tracking mode, un event en erreur est retry avec une back-off period avec un maximum de 60s timeout
* With subscribing mode, une erreur peut être remontée au haut de la chaîne pour rollback la transaction
* Si une exception est lancée dans un command handler, la transaction est rollback
* Si une exception est lancée depuis un commandhandler, même si l'exception est de type Exception, il s'agira une d'exception de type `CommandExecutionException` qui sera lancée
* `@ExceptionHandler` de Axon permet de catcher les exceptions. Il faut coupler l'annotation avec une classe qui implémente `ListenerInvocationErrorHandler` pour qu'Axon soit capable de rollback la transaction. Il faut noter rethrow l'exception. Il faut également enregistrer le listener afin de le binder au processing group.
* CommandInterceptor, EventInterceptor, BeanValidation (with Hibernate validation par exemple)


### Saga/Process
Afin de synchroniser les aggregats, Axon propose les Saga. Le pattern Saga permet de faire des opérations transactionnelles au sein d'une architecture microservices. Il existe deux formes :
* Distribué : si un microservice échoue au traitement d'une commande ou d'un event, il envoie une commande de compensation
* Centralisé : un microservice est responsable de centraliser le process transactionnel et si un service échoue, il est chargé d'envoyer une commande de compensation.

Au sein d'Axon, les Saga sont sous la deuxième forme et servent d'avantage à synchroniser un process. Par exemple, à synchroniser les aggregats au sein d'un même service, même s'il est possible de le faire avec plusieurs services.

#### Process
Une saga est une class portant l'annotation `@Saga`. Elle peut avoir des dépendances, mais celles-ci doivent être `transiant` car les saga sont sérialisées et les dépendances ne doivent pas l'être.

Un process Saga est démarré par une fonction portant l'annotation `@StartSaga`. Cette fonction est généralement déclenchée par un event.

Les méthodes du process Saga sont annotées par des `@SagaEventHandler` et recoivent les différents events du process pour y réagir. Ces méthodes doivent prévoir un comportement de compensation dans le cas où une étape du process échoue.

Pour finir un process Saga, il faut annoter un ou plusieurs méthodes par `@EndSaga` ou manuellement appeler la méthode statique de Saga `end()`. Il est primordial de finir une Saga dans le risque de laisser un process en cours.

Les méthodes de Saga, les annotations, ont le paramètre `associationPropery` qui définir l'attribut de correlation de la Saga. Il est conseillé d'utiliser un aggregate identifier.

#### Deadlines
Une Saga est un process asynchrone qui peut plusieurs du temps à avancer et se terminer. Par exemple, un process de paimeent n'est pas forcément immédiat mais peut dépendre de l'utilisateur qui doit valider la transaction auprès de sa banque.

Il est possible d'annuler une Saga ou de modifier le process dans le cas où une étape dépasse un temps limite. Pour cela, il existe les deadlines. Une deadline est le lancement d'un event dans le cas où un event n'arrive pas. Ainsi on peut démarrer un process de compensation.

Les deadlines peuvent être définie dans les Saga ou les aggregats. Les events des deadlines ne sont pas event sourced et ne sont donc pas enregistrés dans l'event store.

La mise en place d'une deadline requiert un deadline manager. Il existe plusieurs types :
* Simple deadline manager : permet d'avoir les tâches de surveillance en mémoire mais si la JVM redémarre, les dealines sont perdues
* QuartzDeadlineManager : sauvegarde les deadlines en base, ce qui est préférable pour les deadlines qui sont longues. Ce manager s'appuie sur `QuartzScheduler` qui est une dépendance supplémentaire.

La création d'une deadline se fait au sein d'un handler via `deadlineManager.schedule(<timeout>, <deadlineName>, event)`. Le dernier paramètre `event` est un payload qui est transmis au deadline handler. Cela permet d'avoir un context au sein du handler. Pour gérer l'activation d'une deadline, il faut une fonction annotée avec `@DeadlineHandler(deadlineName = <deadlineName>)`. 

La définition d'une deadline retourne un ID qui permet d'annuler une deadline individuellement via la méthode `deadlineManager.cancelSchedule(<deadlineName>, <deadlineId>)`. Il est également possible d'annuler toutes les deadlines portant le même nom via `deadlineManager.cancelAll(<deadlineName>)`. Il est important de noter que toute deadline lancée doit être annulée au risque d'avoir une boucle d'event de deadline lancés.

Attention : le résultat de la deadline doit être traité dans le même fichier (l'event ou la commande lancée par la deadline).


### Query
La lecture de projection peut être fait classiquement ou alors via des utilitaires Axon.

Pour la récupération de projections, Axon propose les `Query` qui sont des POJO envoyés dans une `QueryGateway` et interceptées par des fonctions qui ont l'annotation `@QueryHandler`. Une query peut avoir plusieurs handlers.

#### Subscription query
Etant donné que la query est en eventual consistency, la récupération de projection peut produire des résultat non à jour. Afin de pallier à ce problème, Axon propose les subscription query. Attention, celles-ci ne fonctionnent que si la partie query est dans le même déployable que la partie command.

Une subscription query permet de retourner le résultat courant et si un changement arrive, le client est notifié avec la mise à jour. Et ce jusqu'à ce qu'il y a une annulation de la subscription.

Deux pattern peuvent être mis en place :
* Si le client est un UI
    1. Le client envoie une command
    2. Si la command est en succès, le client envoie une subscription query
    3. Quand le process est fini (par exemple la Saga), la subscription envoie un update
* Si le client n'a pas d'UI (par exemple via Postman ou un client HTTP)
    1. Injecter la query gateway dans le controller de la command
    2. Si la command est en succès, la query gateway envoie une subscription query
    3. La projection à jour est envoyée

Pour notifier des changement dans une projection, il faut utiliser l'objet `QueryUpdateEmitter` et la fonction `queryUpdateEmitter.emit`.


### Replay
Il arrive qu'il y ait besoin de 'rejouer' tous les événements et les renvoyer dans les `@EventHandler`. Plusieurs cas peuvent demander un replay, par exemple :
* Une projection évolue
* Une désynchronisation est arrivée

Axon propose un mécanisme de replay, lancé par des tracking processors. Cela implique que les subscription processors ne peuvent pas être utilisés pour un replay.

Un replay est lancé dans un thread séparé car lancé par un tracking processor. Les Saga ne sont pas 'replayables'.

Pour lancer un replay, il faut utiliser la dépendance `EventProcessingConfiguration` et lancer la méthode `eventProcessingConfiguration.eventProcessor(<processorName>, TrackingProcessor::class)`. Le processor name est un nom de processor définit dans la configuration de l'application. Un processor définit par `@ProcessingGroup` peut être utilisé.

Puis, le process de lancement est :
```
eventProcessingConfiguration.eventProcessor(<processorName>, TrackingProcessor::class)
.map {
    it.shutDown()
    it.resetTokens()
    it.start()
}
```

A noter que le token dans `resetTokens()` est un token de suivi des events. Avant le reset token, celui du processor doit être au niveau du dernier event et à la fin du replay également.

Il est préférable d'utiliser un processor de replay dédié dans le cas ou il échoue et que le token ne soit pas à la fin.

Il est possible d'exclure certains event handler du replay en les annotant par `@DisallowReplay`. Il est également possible d'effectuer des traitement avant le lancement du replay avec une function annotée par `@ResetHandler`.


### Snapshot
Lorsque dans une application, il y a beaucoup d'events, celle-ci peut souffir de problèmes de performance. En effet, à chaque traitement de command, Axon a besoin de connaître l'état courant d'un aggregat. Pour cela, il a besoin de lire tous les événements de l'aggregat afin de construire sont état. Même si Axon garde en mémoire tous les aggregats, il peut y avoir des problématiques de temps de démarrage.

Pour palier à ce problème, Axon propose les snapshots qui sont des états intermédiaires des aggregats. Axon va partir du dernier snapshot d'un aggregat pour le reconstruire. Un snapshot est un event mais qui est stocké dans une table à part.

Les snapshots peuvent être créés selon plusieurs critères :
* A chaque intervalle de temps
* Après N events
* Quand on détecte un chargement d'aggregat dépassant un temps spécifié

Pour créer des snapshots, il faut définir un nom de snapshot et utiliser un snapshotter :
```
@Bean(name = ["productSnapshotTriggerDefinition"])
fun productSnapshotTriggerDefinition(snapshotter: Snapshotter): SnapshotTriggerDefinition {
    // 3 is the number of events after to create a snapshot
    return EventCountSnapshotTriggerDefinition(snapshotter, 3)
}
```

Puis dans la définition de l'aggregat à snapshotter, il faut ajouter le nom du trigger dans son annotation `@Aggregate(snapshotTriggerDefinition = "productSnapshotTriggerDefinition")`.


### Testing
// TODO
(unit, integration, jgiven/acceptance) 
* https://discuss.axoniq.io/t/starting-axon-server-using-testcontainers-from-a-springboottest-port-conflict/2166
* https://androidexample365.com/running-axon-server-in-testcontainers-tests/
* https://github.com/holixon/axon-server-testcontainer
* https://docs.axoniq.io/reference-guide/axon-framework/testing


### Audit trail
Etant donné qu'avec Axon, les applications sont en Event Sourcing, tout l'historique des événements est disponible pour créer un audit trail. Cependant, il n'est pas conseillé de lire directement l'event store pour cela. Une meilleure pratique est de créer une projection pour l'audit trail.


### Configuration
// TODO


# NOTES
* A tester, est-ce que le replay lit tous les événements, mêmes ceux qui ne sont pas dans le processing group ? ou alors il faut lancer le replay de tous les processing group ?
* Exemple sous forme de tuto à faire
