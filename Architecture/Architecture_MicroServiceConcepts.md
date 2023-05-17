# Micro-services concepts

## Communication
Pour faire communiquer les services entre eux, il est possible d'utiliser des bus de message comme Kafka et RabbitMQ ainsi que des requêtes HTTP

## Transaction
Pour gérer le transactionnel dans une architecture microservices, il faut utiliser le pattern Saga

### Choreaography based Saga
* Le principe est que les services communiquent via un message broker
* Celui qui lance la transaction, envoie un premier message, les suivants se synchronisent via une série de messages en fonction des traitements. Cette chaîne représente une transaction. Si une erreur arrive, les services doivent publier un événement de compensation afin de rollback la transaction. Le service qui a l'erreur publie un premier message et la chaîne est traitée dans l'ordre inverse afin que chaque micro service annule la transaction.

### Orchestration based Saga
* Un service contient une class Saga, c'est celui-ci qui va orchestrer la transaction. Ce service d'orchestration traite l'ensemble des messages suite aux différents traitements et republie des messages pour orchestrer les autres services.
* C'est le service orchestrateur qui va lancer le rollback dans le cas où il y a une erreur quelque part. Il va publier des messages dans l'ordre inverse.