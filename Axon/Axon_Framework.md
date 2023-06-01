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