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
* Pour faire des opérations sur les messages avant leur réception comme de la valition
* Il s'agit de class qui implémentent `MessageDispatchInterceptor`
