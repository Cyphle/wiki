# Les ressources RH

Une équipe web est typiquement composée des rôles suivants :
* Fonctionnels
    * PO
    * QA
* Techniques
    * Tech lead
    * Développeur
    * SRE (Site Reability Engineer ou DevOps)

En fonction de la stratégie produit, il est important de faire attention à comment former ces équipes.

## Considérations d'expérience
L'expérience d'un profil n'indique pas forcément son niveau d'expertise mais est un premier indicateur.

Afin de faire évoluer au mieux une équipe, il est préférable d'avoir d'avantage de profils expérimentés que de juniors afin que les expérimentés puissent faire avancer le projet en même temps que former les juniors au lieu de passer leur temps à former les juniors.

## Considérations de profils

### Les profils fonctionnels
Un PO est un rôle très complexe dans une organisation Agile. En effet, il est responsable du produit et a donc généralement en charge :
* La conception
* Le suivi du delivery
* Le suivi de l'utilisation du produit
* L'exploration fonctionnel
* La circulation du fonctionnel au sein de l'équipe
* Les tests fonctionnels s'il n'y a pas de PO
Il s'agit donc d'un rôle clé dans le suivi de la roadmap et la stratégie globale. Bien souvent, les profils PO ont une expérience et des compétences en terme d'agilité et de conception mais peu en terme de gestion de projet.

En effet, pour rappel, un projet agile fixe le temps et le coût et utilise le contenu comme variable d'ajustement. C'est pourquoi il est important pour un PO de pouvoir se projeter à plus long terme que quelques sprints.

Un PO doit également maîtriser des techniques de conception ainsi que des techniques d'exploration du besoin comme le Story Mapping, l'Event Storming, l'Impact Mapping, etc.

### Les profils techniques
#### Développeurs
Selon la stratégie d'équipe, les développeurs seront recrutés en fonction de leur spécialité :
* Full-stack
* Front
* Back
Le choix des profils a chacun ses avantages :
* Full-stack: ces profils pourront développer les features de bout en bout sans discontinuité de développement. Cependant, de manière général, ils ont toujours une préférence soit pour le back, soit pour le front.
* Back et Front: avoir des profils séparés pour le front et le back permet d'avoir des experts par technologie ce qui permettra le développement de solutions plus propres et évolutives. Cependant, il peut devenir difficile de synchroniser les développements entre front et back ainsi que d'avoir suffisamment de travail pour tous les profils. En effet, il peut arriver que certains projets soient très front ou très back.

Il est également important de savoir comment créer une cohésion ainsi qu'une cohérence de l'équipe en offrant par exemple un environnement technique challengeant ou tout le monde pourra évoluer et faire évoluer les autres. Actuellement, un des facteurs qui permet ceci est le Software Craftsmanship.

#### SRE
Les SRE (ou DevOps dans le language courant) ont un rôle primordial. Il ne s'agit plus des classique administrateurs systèmes proposant un service de serveurs et de déploiement. Ils doivent faire parti intégrante de l'équipe. L'automatisation des tâches est leur rôle principal. Cela inclue :
* La mise en place d'un outil de CI/CD (les pipelines de déploiments doivent être fait en collaboration avec les développeurs)
* La mise en place d'outils de monitoring infrastructure et applicatif
* Les outils d'automatisation comme Terraform, Helm, etc

Ils ont également un rôle important dans la sécurité en mettant en place les outils nécessaires ainsi que certaines politiques d'accès.

A noter qu'avec le développement des cloud providers comme AWS et Azure, le démarquage entre SRE et développeurs est plus flou et les deux rôles doivent travailler en étroite collaboration.

## Considération de mindset
* Il faut éviter les héros. Bien entendu, il est important de savoir s'entourer d'experts dans certaines technologies mais créer des silos est toujours sujet à créer des SPOF (Single Point of Failure).
* Il est également important de savoir créer une ambiance d'équipe où tout le monde peut s'entraider et évoluer ensemble.
    * D'une part cela permet de challenger en permanence ce qui est en place et de faire évoluer les produits dans une meilleure direction
    * D'autre part cela permet de motiver d'avantage les techniciens