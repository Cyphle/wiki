# Axon framework

## Command
Pour envoyer les commands, il faut injecter une command gateway d'Axon. Les command gateway permettent d'envoyer les commandes via :
* send : qui retourne une future
* sendAndWait : qui attend que le traitement de la commande soit fini

## Aggregate
* Un aggregat est identifié via un AggregateIdentifier unique
* Un aggregat est l'objet managé par Axon. C'est lui qui contient les command handlers qui traitent les commandes
* Il contient également les event sourcing handler qui permettent de réhydrater l'objet à partir des événements
* Un aggregat doit avoir un constructeur vide
* Pour envoyer un événement depuis l'aggregat, il faut utiliser `AggregateLifecycle.apply(event)`

## Anotations
* TargetAggregateIdentifier
* AggregateIdentifier
* Aggregate
* CommandHandler
* EventSourcingHandler  

## Message Dispatch Interceptor
* Pour faire des opérations sur les messages avant leur réception comme de la valition de command
* Il s'agit de class qui implémentent `MessageDispatchInterceptor`

## Set based consistency validation
* Pour vérifier qu'un aggregat n'existe pas déjà, il est possible de mettre en place un pattern avec une table contenant le nécessaire de vérification comme l'aggregate identifier et les champs qui doivent être uniques. Lors de la réception d'une command de création, il faut aller regarder cette table.

## Event handling
* Si un événement est handle par plusieurs handlers, il est possible de les grouper dans un unique processing group avec `@ProcessingGroup("product-group")`. C'est pour que handlers soient tous dans le même thread

## Event processor
* Tracking mode (default): pull their messages from a source using a thread that it manages itself in a different thread
* Subscribing mode : subscribe themselves to a source of events and are invoked by thread managed by the publishing mechanism in the same thread

## Error handling
* With tracking mode, un event en erreur est retry avec une back-off period avec un maximum de 60s timeout
* With subscribing mode, une erreur peut être remontée au haut de la chaîne pour rollback la transaction
* Si une exception est lancée dans un command handler, la transaction est rollback
* Si une exception est lancée depuis un commandhandler, même si l'exception est de type Exception, il s'agira une d'exception de type `CommandExecutionException` qui sera lancée
* `@ExceptionHandler` de Axon permet de catcher les exceptions. Il faut coupler l'annotation avec une classe qui implémente `ListenerInvocationErrorHandler` pour qu'Axon soit capable de rollback la transaction. Il faut noter rethrow l'exception. Il faut également enregistrer le listener afin de le binder au processing group.

## Saga
* Pattern pour gérer les transactions en micro service. Cela permet par exemple de synchroniser une transaction et de la rollback
* Une sage a un début `@StartSage` et une fin `@EndSaga`. Le `@EndSaga` doit être sur un `@SagaEventHandler`
* Les `@SagaEventHandler` définissent des `associationProperty` qui permettent d'associer l'instance de saga avec l'objet (préférer l'aggregate identifier)
* Les dépendances doivent être transient pour éviter qu'elles soient sérialisées car les saga sont sérialisées

## Deadlines
* Les deadlines permettent de gérer le temps pour finir une saga ou la faire avancer. Deadline is an Event that takes place of an absence of an event.
* Par exemple, si on attend un événement dans les x heures et qu'il n'arrive pas, une deadline peut être trigger pour démarrer un flow de compensation
* Les deadlines peuvent être dans les saga ou les aggregats
* Trigger a state change or a command
* Ce n'est pas sourced : un deadline event n'est pas enregistré dans l'event store
* Deadlinemanager
    * Simple deadline manager : keeps scheduled tasks in memory (si la jvm est redémarrée, les deadlines sont perdues)
    * QuartzDeadlineManager : sauvegarde les deadlines en base. préféreable pour les deadlines longues par exemple 72 heures. S'appuie sur QuartzScheduler qui est hors Axon mais une dépendance supplémentaire
* `deadlineManager.schedule(<time>, "deadline-name", event)` & `@DeadlineHandler(deadlineName = "deadline-name")`
* Deadline scope : le scope où le résultat de la deadline doit être traité (par défaut dans le même fichier)
* Il ne faut pas oublier de cancel les deadlines sinon elles peuvent se redeclencher
* La déclaration d'une deadline retourne un ID qui permet de cancel une deadline en particulier

## Subscription queries
* En CQRS nous sommes en eventual consistency
* Subscription queries permet de retourner le résultat courant et si un changement arrive, notifie le client avec l'update. Ceci jusqu'à ce qu'il y ait un cancel de subscription
* Par exemple
    * S'il y a une UI,
        1. client envoie une command
        2. si la command est successfull, envoyer une subscription query
        3. quand le saga flow a fini
        4. envoyer une update
    * Si no UI (par exemple avec postman)
        1. Inject query gateway dans le controller
        2. Si la command est successfull, query gateway envoie une subcription query
        3. Envoie la data à jour
* L'object `QueryUpdateEmitter` permet d'émettre des changements liés pour une query avec `queryUpdateEmitter.emit`

## Snapshots
* Permet de créer une image tous les x events d'un aggregat
* Un snapshot est un event
* Paramétrage :
    * regular intervals
    * after x events
    * when loading takes longer than specified time

## Message correlation
* il est possible de correler les messages entre eux comme les events et les query responses et les commands
* les correletion id se trouvent dans les meta data des messages
* pour générer il faut utiliser des `CorrelationDataProvider`
* Par defaut c'est `MessageOriginProvider` qui créé des correlationId et traceId
    * correlationId référence le message parent
    * traceId référence le message root
* pour créer des custom provider il faut implémenter `CorrelationDataProvider`
* il existe des provider 
    * SimpleCorrelationDataProvider
    * MultiCorrelationDataProvider qui merge des prvoviders

## Messages au sein d'Axon
* Tout est message
* ils ont des meta data
* les commands sont wrappes dans des CommandMessage
* les events wrappés dans des EventMessage
* si c'est un aggregat qui publie un event, c'est un DomainEventMessage
* Une query peut avoir plusieurs handler
* Une command doit avoir un unique handler
* Les events doivent être serializables

## Replay
* Le principe est de relire tous les events du store pour relancer tous les `@EventHandler` et reconstruire les projections
* Supporté par le Tracking Event Processor
* Possible de rajouter des `@DisallowReplay` pour exlure des event handler du replay
* Avant de lancer un replay, il faut arrêter le tracking event processor pour éviter qu'il soit dans un état non départ
* `@ResetHandler` permet de lancer une action avant le début du replay
* Les sagas ne sont pas replaybale par défaut
* Les event processor subscribing ne peuvent être utilisé pour du replay

## Notes
* Pour lire les événements d'un aggregat ou vérifier s'il existe pas déjà, essayer
```
@Bean
    fun test(eventStore: EventStore) {
        eventStore.readEvents(aggregateIdentifier)
    }
```
* TODO : il faut faire des notes avec ce qu'il faut savoir dans l'ordre (naming important, concepts, etc)
* Titres pour des notes plus précises
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
* Dans un pattern cqrs évent source axon, il faut pouvoir envoyer des commandes aux aggregats par id. ça veut dire que les projections contiennnent le nécessaire pour connaître tous les ids

•	Comment on fait si on a plusieurs aggregats ?
•	Comment les synchroniser ? Est-ce qu’utiliser des saga c’est recommandé ?
> C'est un process. On peut utiliser les Saga de Axon qui sont asynchrone et sont un process


•	Comment envoyer des opérations en batch (création, update, etc exemple avec les arborescences) ? Pour des optimisations notamment par exemple créer des objets en masse partie command ET partie query
> Normalement il n'y a pas besoin de faire de batch
> A tester avec un traitement unitaire
> Pour la partie projection, on peut augmenter le segment du batch (micro batch)


•	Comment peut-on se passer de set based validation ? Ca demande d’avoir une base de données supplémentaire
> Contrainte d'unicité : les attributs d'unicité, il faut les sharder, cela devient une clé qui devient l'id d'aggregat (qui n'est plus généré aléatoirement) (cf exemple github.com/karamelsoft/axon-bank customer constraint)


•	Est-ce qu’on doit avoir tous les id en front pour envoyer dans la command ? Par exemple, je veux update tous les trucs qui ont tel attribut ou je veux créer une arborescence à tous mes actifs mais mes actifs sont déjà créés
> On peut faire des commandes qui peuvent ne pas être liées à un aggregat. Il faut une routing key (@Routing). Il s'agit d'une vraie saga. La routing key représente la région.
> Il faut utilsier un process qui appelle la query


•	Comment lire les events pour faire un audit trail ? Est-ce que c’est recommandé ?
> Il ne faut pas attaquer directement l'event store directement pour des questions de perf. Il faut utiliser des eventhandler ou eventinterceptor pour les projeter
> Le store n'est pas une vue


Un process (@Saga) est quelque chose qui se passe suite à un événement métier. Ce n'est pas de la commande. C'est de la synchro.

On doit pas avoir besoin de faire de find aggregate


Il faut bien avoir des read et write déployés indépendamment (le write doit être shardé et le read doit être répliqué)

* tester si y a plusieurs tracking processor, si on lance un replay, comment ça se pases