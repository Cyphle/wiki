# Technologies

Dans ce chapitre ne sera traité qu'une stratégie web avec des front s'appuyant sur des technologies SPA (Single Page Application).

## Front
Avec l'avènement des technologies pour générer des SPA, il est aujourd'hui assez simple de trouver de la ressource notamment concernant les principaux acteurs du marché Angular, React et VueJS.

Attention toutefois à l'écosystème. Par exemple Angular est un framework et vient avec de nombreux outils et impose une architecture. Tandis que React est une librairie de rendue, même si elle s'enrichit de plus en plus pour s'éloigner de plus en plus d'une simple librairie de rendue. Il faut donc ajouter un écosystème à React qu'il soit "complet". Il s'agit ici de son avantage et de son inconvénient.

Pour la gestion des états front, il est possible d'utiliser des outils gérant le state management. Les plus connus implémentent le pattern Redux. On retrouve notamment :
* NgRxStore pour Angular
* Redux pour React
* Les hooks de React, qui n'implémentent pas le pattern Redux mais permettent de s'en approcher et de faire du state management plutôt sur du pattern Flux.
* VueX pour VueJS

Le choix de la technologie front n'importe pas dans le process d'industrialisation car elles tournent sur Javascript.

Attention, ce n'est pas forcément le cas pour d'autre technologie comme Flutter et Dart.

## Back
Le choix des outils back est primordial. Il faut vraiment étudier l'écosystème et la communauté des technologies avant de les utiliser.

Quelques éléments d'écosystème à vérifier :
- ORM
- Logging
- Frameworks web
- Frameworks d'injection de dépendances
- Gestion des ressources CPU et RAM
- Performances
- Communauté
- Courbe d'apprentissage
- Ouverture à d'autres composants
- Maintenance : est-ce que l'outil est maintenu par exemple
- Sécurité
- Licence

## Déploiement
* Conteneur ou pas ? avatanges et inconvénients
* Kubernetes
* CI/CD

## Cloud
* Cloud ou pas ?

## MCO : Maintien en conditions opérationnelles
* Monitoring
* Logging
* Performances
* Scalabilité
* PRA/PCA

## Comment choisir ?
* Par rapport au marché
* Par rapport à la maturité et écosystème
* Par préférence personnelle