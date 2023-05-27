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

## Notes
* Pour lire les événements d'un aggregat ou vérifier s'il existe pas déjà, essayer
```
@Bean
    fun test(eventStore: EventStore) {
        eventStore.readEvents(aggregateIdentifier)
    }
```
* TODO : il faut faire des notes avec ce qu'il faut savoir dans l'ordre (naming important, concepts, etc)
