# Les flux de travail

## Agilité
Aujourd'hui l'utilisation des frameworks Agile dans le monde de l'informatique est quasiment un standard. Toutefois il faut faire attention à bien l'utiliser. Aucun outil n'est une solution miracle, utilisé à mauvais escient un outil peut provoquer plus de dégats qu'apporter des solutions.

Le principe fondamental de l'agilité est le continuous improvement.

### Planification
Tout projet et en particulier en informatique comporte trois axes projets :
* Le coût
* Le temps
* Le contenu

Le principe d'une planification Agile, en particulier Scrum puisqu'il s'agit du framework très utilisé aujourd'hui, est de fixer le temps ainsi que le coût du projet et faire varier le contenu. Ce principe peut être mis en opposition à d'autre méthodologie comme le cycle en V où il s'agit du contenu et du coût qui sont fixés et le temps comme variable. A noter qu'il est impossible de fixer les trois axes.

Les items de contenu d'un projet sont découpés selon trois niveaux :
* Les user stories : éléments les plus petits définissant une fonctionnalité. Dans l'idéal, une story respecte le principe INVEST (Independant, Negociable, Valuable, Estimable, Small, Testable). Cela n'est pas toujours possible comme pour les sujets très techniques.
* Les Epics : éléments d'ordre plus gros regroupant plusieurs user stories dans un même groupe cohérent de fonctionnalités
* Les Thèmes : éléments d'ordre encore plus gros regroupant les epics dans un même groupe cohérent de fonctionnalités

La planification avec Scrum joue sur plusieurs temps :
* Au niveau Sprint : permet de prévoir des petites avancées du projet et permettant de faire des rétrospective sur le travail réalisé à interval régulier
* Au niveau Epics : permet de se projeter sur plusieurs mois
* Au niveau Thèmes : permet de se projeter sur plusieurs mois vers plusieurs années

La planification repose principalement sur la vélocité de l'équipe. Il est donc important qu'elle soit stable. Il faut tout de même prendre en compte les aléas de l'équipe (congés, turn over, etc). Il s'agit d'une valeur estimative de delivery des stories. L'utilisation de points de vélocité permet d'estimer une complexité à délivrer une feature. En effet, la complexité d'une story n'est pas forcément directement liée au développement mais peut être par exemple liée à sa validation fonctionnelle (par exemple les systèmes d'habilitations peuvent être très simples à mettre en place mais à valider cela demande de créer de nombreux cas).

L'idéal est d'estimer les stories, les epics ainsi que les thèmes afin de pouvoir se projeter loin dans le temps. Puis le fait d'avancer sprint par sprint, cela permet de découper au fur et à mesure les epics et les thèmes et d'ajuster soit la date d'atterissage finale soit le contenu.

### Construction itérative
Il est important de construire un logiciel de manière itérative. Un logiciel doit être évolutif et adaptable. Il ne s'agit pas de recommencer mais bien de pouvoir faire évoluer le logiciel en fonction des besoins qui sont souvent fluctuents.

C'est pourquoi il est primordial de savoir définir des MVP (Minimum Viable Product) ou MMP (Minimum Marketable Product) afin de pouvoir lancer les développement le plus tôt possible et récolter des feedbacks le plus tôt possible.

Lancer une conception jusqu'à obtenir le produit fini puis lancer un développement itératif revient à faire du cycle en V ou waterfall.

## Software Craftsmanship
Il est aujourd'hui difficile de trouver des développeurs et encore plus des développeurs qui vont construire des solutions évolutives et maintenables.

Afin de se démarquer, ces développeurs se présentent comme des Software Craftsmen, des artisans du logiciel. Concrètement cela signifie qu'il s'agit de développeurs maîtrisant des principes de développement permettant d'avoir des logiciels :
* Evolutifs
* Facilements compréhensibles (notamment pour les autres développeurs)
* Maintenables
* Répondant strictement aux besoins métiers

Parmi les principes on retrouve notamment :
* Les architectures : micro-services, hexagonal, CQRS, lambda core, ...
* Les patterns : patterns du GOF, patterns d'entreprise, ...
* Les principes : SOLID, 4 rules of simple design, KISS, DRY, ...
* Les techniques de conception et design : DDD, pyramide de tests, ...
* Les techniques de travail : TDD, BDD, ...

Il est également important de ne pas faire uniquement attention aux compétences logicielles mais également aux softskills. Une équipe est un groupe d'individus qui travailleront mieux et évoluerons mieux s'ils arrivent à travailler ensemble, qu'ils s'entre aident et qu'ils partagent les mêmes valeurs.

## DevSecOps
Avec les outils actuels (AWS, Kubernetes, ...) le monde des opérateurs d'infrastructure et le monde des développeurs sont de moins en moins distincts et doivent être de plus en plus mélangés afin que les différents profils travaillent conjointement.

Il est donc intéressant de faire appel à des SRE qui connaissent bien les outils cloud, CI/CD, et autres outils d'automatisation et monitoring ainsi que des développeurs qui n'ont pas peur voire ont une apétence pour les outils d'industialisation.

Egalement, dans un monde de plus en plus web, il est primordial que chacun soit un minimum sensibilisé à la sécurité voire maîtrise les concepts primordiaux de chiffrement, de flux réseaux, etc.