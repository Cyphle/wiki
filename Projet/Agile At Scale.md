# Agile At Scale
## **1. Coordination entre équipes**
**a) Scrum of Scrums (SoS)**
- Une réunion régulière regroupant des représentants de chaque équipe Scrum.
- Objectif : Identifier et résoudre les interdépendances et les obstacles entre les équipes.
- Fréquence : Quotidienne ou hebdomadaire.
- Participants : Scrum Masters ou délégués.

**b) Dépendance Mapping**
- Identification et visualisation des dépendances entre équipes ou projets.
- Techniques :
	- **Dependency Matrix** : Tableau montrant les interactions entre équipes.
	- **Dependency Board** : Tableau Kanban spécifique pour gérer les dépendances.

**c) Synchronisation des cadences**
- Toutes les équipes partagent une même durée de sprint (par exemple, 2 semaines).
- Les **plannings et livraisons** sont alignés pour faciliter l’intégration et les livraisons.

**c) Architecture microservices et DDD**
Certains architectures techniques permettent de réduire les dépendances entre les équipes. Par exemple le DDD et l'architecture micro services permet de former des équipes en charge de domaines métiers identifiés et le plus autonomes possible. Cela permet aux équipes de réduire les dépendances organisationnelles comme synchroniser les livraisons et les sprints. Les dépendances sont décalées à des dépendances techniques où les équipes doivent mettre en place des pratiques et solutions pour que les interactions entre les différentes parties logicielles fonctionnent en permanence.

## **2. Gestion des priorités**
**a) Vision Produit Partagée**
- Une vision claire, définie par un **Product Manager** ou un **Chief Product Owner**, guide toutes les équipes.
- Techniques :
	- **Elevator Pitch** : Résumer la vision en une phrase concise.
	- **Product Vision Board** : Document central pour la mission, les objectifs et la cible utilisateur.

**b) Backlogs hiérarchisés**
- Les niveaux de backlog :
	- **Portfolio Backlog** : Vision stratégique et initiatives clés.
	- **Program Backlog** : Priorités inter-équipes.
- **Team Backlog** : Tâches spécifiques aux équipes.
- Techniques de priorisation :
	- **WSJF (Weighted Shortest Job First)** : Priorisation basée sur la valeur et l’urgence.
	- **MoSCoW** : Catégoriser les tâches en Must-Have, Should-Have, Could-Have, Won’t-Have.

**c) Alignement des Roadmaps**
- Une roadmap unifiée pour toutes les équipes :
- Définit les grandes étapes et priorités.
- Facilite la communication entre les équipes et les parties prenantes.

## **3. Gestion des livraisons**
**a) Incréments de Programme (PI)**
- Technique utilisée dans SAFe pour planifier des livraisons sur une période donnée (8-12 semaines).
- Inclut toutes les équipes d’un Agile Release Train (ART).
- Objectif : Synchroniser les efforts pour livrer un incrément cohérent.

**b) Livraison incrémentale**
- Chaque sprint produit des incréments fonctionnels qui peuvent être testés ou mis en production.
- Techniques associées :
	- **Feature Toggles** : Activation/désactivation de fonctionnalités en production.
	- **Canary Releases** : Déploiement progressif sur un petit pourcentage d’utilisateurs.  

**c) Continuous Delivery Pipeline (CDP)**
- Automatisation des tests, intégration et déploiements via DevOps.
- Étapes principales :
	1. **Continuous Exploration** : Identifier ce qui doit être construit.
	2. **Continuous Integration** : Développer et tester.
	3. **Continuous Deployment** : Mettre en production rapidement.

## **4. Techniques collaboratives**
**a) PI Planning (Program Increment Planning)**
- Réunion de planification collaborative avec toutes les équipes, utilisée dans SAFe.
- Objectifs :
	- Définir les priorités pour le prochain incrément de programme.
	- Identifier et résoudre les dépendances.

**b) Communities of Practice (CoPs)**
- Groupes de partage de connaissances entre membres ayant des compétences ou rôles similaires.
- Exemples :
	- CoP des Scrum Masters pour améliorer les pratiques agiles.
	- CoP des Développeurs pour partager des solutions techniques.

**c) Ateliers collaboratifs**
- Sessions structurées pour résoudre des problèmes complexes ou aligner les parties prenantes.
- Exemples :
	- **Design Thinking Workshops** : Prototyper rapidement des idées.
	- **Event Storming** : Cartographier les processus métier.

## **5. Gouvernance Agile**
**a) Metrics et KPIs**
- Suivi de la performance à grande échelle.
- Exemples :
	- **Lead Time** : Temps nécessaire pour passer une fonctionnalité de l’idée à la livraison.
	- **Velocity Aggregée** : Vélocité totale des équipes combinées.
	- **Customer Satisfaction** : Mesure de la satisfaction utilisateur.

**b) Agile Portfolio Management**
- Gestion des investissements et des priorités à l’échelle du portefeuille de produits.
- Outils :
	- **Portfolio Kanban** : Suivi des initiatives stratégiques.
	- **Epic Hypothesis Statement** : Décrire les grandes idées stratégiques.

**c) Servant Leadership**
- Les managers agiles agissent comme des facilitateurs, soutenant les équipes dans la suppression des obstacles.

## **6. Techniques pour l’amélioration continue**
**a) Inspect & Adapt (I&A)**
- Sessions rétrospectives à l’échelle pour évaluer la performance globale du programme ou de l’organisation.
- Objectifs :
	- Identifier les points faibles.
	- Planifier des actions correctives.

**b) Shared Retrospectives**
- Rétrospectives inter-équipes pour améliorer la collaboration et résoudre les problèmes communs.

**c) Growth Hacking Agile**
- Techniques rapides pour expérimenter des solutions :
- **A/B Testing** : Tester plusieurs variantes pour optimiser les résultats.
- **Hypothèse & Validation** : Tester des idées rapidement et pivoter si nécessaire.

## **7. Intégration technique (DevOps)**
**a) Automatisation des tests**
- Tests automatisés pour garantir que les incréments produits sont prêts pour l’intégration.

**b) Intégration Continue (CI)**
- Les équipes intègrent fréquemment leur code dans un dépôt partagé pour détecter les conflits tôt.

**c) Delivery Continue (CD)**
- Automatisation des déploiements pour livrer des fonctionnalités rapidement et en toute sécurité.

## Résumé

| Domain                         | Technique clé                                    |
| :----------------------------- | :----------------------------------------------- |
| **Coordination inter-équipes** | Scrum of Scrums, PI Planning, Dépendance Mapping |
| **Priorisation et stratégie**  | Roadmaps partagés, WSJF, Portfolio Kanban        |
| **Livraison et qualité**       | Continuous Delivery, Feature Toggles, CDP        |
| **Collaboration**              | Communities of Practice, Ateliers, I&A           |
| **Gouvernance et alignement**  | Agile Portfolio Management, KPIs, Metrics        |

# Frameworks

Il existe plusieurs frameworks d’**Agile at Scale**, comme **SAFe**, conçus pour appliquer les principes Agile à grande échelle dans des organisations complexes. Ces frameworks permettent de coordonner plusieurs équipes Agile, d’aligner les objectifs stratégiques, et d’optimiser la livraison de valeur. Voici un aperçu des principaux frameworks :

## **1. SAFe (Scaled Agile Framework)**
- **Description** : SAFe est un cadre structuré et complet pour appliquer l’agilité à l’échelle de l’organisation. Il s’articule autour de quatre niveaux : équipe, programme, solution, et portefeuille.
- **Caractéristiques principales** :
	- **Program Increment (PI) Planning** : Planification collaborative sur plusieurs sprints (8-12 semaines).
	- **Agile Release Train (ART)** : Groupes de plusieurs équipes synchronisées travaillant vers un objectif commun.
- Gestion de portefeuille Agile alignée sur les objectifs stratégiques.
- **Avantages** :
	- Très structuré, adapté aux grandes entreprises.
	- Aligne les équipes sur les priorités stratégiques.
- **Inconvénients** :
	- Complexité et lourdeur pour les petites organisations.
- **Utilisation typique** : Grandes entreprises avec des projets multidimensionnels (ex. : banques, entreprises technologiques).

## **2. LeSS (Large-Scale Scrum)**
- **Description** : LeSS étend Scrum à l’échelle en gardant une structure simple. Il est conçu pour 2 à 8 équipes travaillant sur un produit unique.
- **Caractéristiques principales** :
	- **Product Backlog unique** : Une seule liste de priorités pour toutes les équipes.
	- **Sprint synchronisé** : Toutes les équipes démarrent et terminent leurs sprints en même temps.
	- **Shared Definition of Done** : Critères de complétion communs à toutes les équipes.
- **Avantages** :
	- Minimaliste, centré sur les valeurs Scrum.
	- Simple à comprendre et à adopter.
- **Inconvénients** :
	- Moins adapté aux très grandes organisations (>8 équipes).
	- **Utilisation typique** : Entreprises de taille moyenne avec un produit unique.

## **3. Scrum@Scale**
- **Description** : Développé par Jeff Sutherland, co-créateur de Scrum, ce framework étend Scrum tout en préservant sa flexibilité et sa légèreté.
- **Caractéristiques principales** :
	- **Scrum of Scrums (SoS)** : Réunions entre représentants d’équipes pour synchroniser les efforts et résoudre les dépendances.
	- **MetaScrum** : Réunion des parties prenantes pour prioriser et définir les objectifs.
- **Cycles parallèles** :
	- **Cycle de livraison** : Axé sur l’exécution.
	- **Cycle de priorisation** : Axé sur la stratégie et les objectifs.
- **Avantages** :
	- Modulaire et adaptable aux besoins spécifiques.
	- Idéal pour des environnements évolutifs.
- **Inconvénients** :
	- Nécessite une bonne maîtrise de Scrum pour une mise en œuvre réussie.
- **Utilisation typique** : Organismes ayant besoin d’une solution modulaire pour coordonner plusieurs équipes.

## **4. Nexus**
- **Description** : Nexus est une extension légère de Scrum pour coordonner jusqu’à 9 équipes travaillant sur un produit unique.
- **Caractéristiques principales** :
	- **Nexus Integration Team (NIT)** : Équipe responsable de la supervision et de l’intégration des livraisons des différentes équipes.
	- **Integrated Increment** : Livraisons synchronisées intégrées à la fin de chaque sprint.
	- **Nexus Sprint Planning** : Planification des dépendances et des intégrations en début de sprint.
- **Avantages** :
	- Structure simple, proche de Scrum.
	- Bon équilibre entre flexibilité et coordination.
- **Inconvénients** :
	- Limité à un produit ou un domaine.
	- **Utilisation typique** : Entreprises ayant jusqu’à 9 équipes Scrum travaillant sur un seul produit.

## **5. Disciplined Agile (DA)**
- **Description** : DA est un framework adaptatif qui fournit des options pour choisir les pratiques Agile les plus adaptées à un contexte donné.
- **Caractéristiques principales** :
	- Orienté sur le **choix contextuel** : Propose des processus pour des situations spécifiques.
	- Combinaison d’Agile, Lean et DevOps.
	- Modèle basé sur des “processus-goals” qui offrent des recommandations pour chaque phase (planification, exécution, livraison).
- **Avantages** :
	- Flexible, personnalisable pour les organisations complexes.
	- Supporte divers environnements techniques et culturels.
- **Inconvénients** :
	- Courbe d’apprentissage élevée.
	- Moins structuré que SAFe.
- **Utilisation typique** : Grandes organisations avec des besoins variés et des processus diversifiés.

## **6. Spotify Model**
- **Description** : Inspiré par les pratiques de Spotify, ce modèle met l’accent sur l’autonomie des équipes et la culture d’innovation.
- **Caractéristiques principales** :
	- **Squads** : Petites équipes Agile autonomes travaillant sur un aspect spécifique d’un produit.
	- **Tribes** : Groupes de squads travaillant dans un domaine commun.
- **Chapters** et **Guilds** : Structures transversales pour partager des connaissances et des compétences.
- **Avantages** :
	- Favorise l’autonomie, la collaboration et l’innovation.
	- Léger et adaptable.
- **Inconvénients** :
	- Pas un framework formel, nécessite une culture Agile forte.
- **Utilisation typique** : Entreprises technologiques, startups, environnements très collaboratifs.

## **7. Enterprise Kanban (ex. SAFe Kanban, Portfolio Kanban)**
- **Description** : Utilise Kanban pour gérer les flux de travail à l’échelle de l’organisation.
- **Caractéristiques principales** :
	- Visualisation des workflows à tous les niveaux (équipe, programme, portefeuille).
	- Limitation du travail en cours (_Work In Progress_ - WIP).
	- Amélioration continue basée sur les métriques (lead time, throughput).
- **Avantages** :
	- Simplicité et transparence.
	- Adaptable à presque tous les environnements Agile.
- **Inconvénients** :
	- Moins adapté à des projets nécessitant une forte coordination inter-équipes.
- **Utilisation typique** : Organisations Lean ou ayant une approche basée sur les flux de valeur.

## Résumé

| **Framework**         | **Échelle**              | **Structure**                 | **Complexité**   | **Focus**                             |
| :-------------------- | :----------------------- | ----------------------------- | ---------------- | ------------------------------------- |
| **SAFe**              | Très grande échelle      | Structurée (4 niveaux)        | Élevée           | Gestion stratégique et opérationnelle |
| **LeSS**              | Moyenne à grande échelle | Minimaliste                   | Modérée          | Simplification de Scrum               |
| **Scrum@Scale**       | Moyenne à grande échelle | Modulaire                     | Modérée          | Flexibilité et modularité             |
| **Nexus**             | Moyenne échelle          | Structurée (1 produit)        | Faible           | Intégration et coordination           |
| **Disciplined Agile** | Grande échelle           | Adaptative                    | Élevée           | Personnalisation des processus        |
| **Spotify Model**     | Moyenne à grande échelle | Culture et collaboration      | Faible à modérée | Autonomie et innovation               |
| **Enterprise Kanban** | Toute échelle            | Basée sur les flux de travail | Faible           | Amélioration continue                 |

# PI Planning
Le **PI Planning** (Program Increment Planning) est un événement clé du **framework SAFe (Scaled Agile Framework)**. Il s’agit d’une session de planification à grande échelle qui réunit toutes les équipes Agile d’un **Agile Release Train (ART)** pour définir les objectifs et les priorités sur un **increment de programme** (PI), une période de 8 à 12 semaines.

## **Objectifs du PI Planning**
1. **Alignement stratégique** : Aligner toutes les équipes sur les objectifs commerciaux, les priorités et la direction générale.
2. **Planification inter-équipes** : Identifier et résoudre les dépendances entre équipes.
3. **Vision partagée** : Communiquer la vision du produit ou du programme et les objectifs à atteindre pendant le PI.
4. **Engagement des équipes** : Obtenir un engagement collectif sur les objectifs et les livrables.
5. **Collaboration** : Renforcer les interactions entre les équipes, les Product Owners (PO), les Scrum Masters et les parties prenantes.

## **Déroulement d’un PI Planning**
Le PI Planning dure généralement **deux jours consécutifs** et suit une structure bien définie :

### **Jour 1 : Préparation et vision**
1. **Présentation de la vision et des objectifs (Business Context)** :
- L’équipe dirigeante (Product Manager, Solution Architect, etc.) expose la vision stratégique, les objectifs commerciaux et les priorités.
- Un cadre général est donné pour guider les équipes.

2. **Présentation du backlog de programme** :
- Les principales fonctionnalités (_features_) prévues pour le PI sont présentées.
- Les équipes reçoivent les priorités pour organiser leur travail.

3. **Planification initiale par équipe** :
- Chaque équipe Agile analyse les fonctionnalités assignées et commence à planifier son travail.
- Les équipes identifient les dépendances avec d’autres équipes.

### **Jour 2 : Ajustement et finalisation**
4. **Synthèse des plans inter-équipes** :
- Chaque équipe partage son plan avec le reste du train Agile.
- Les dépendances critiques et les risques sont discutés dans un cadre collectif.

5. **Planification des objectifs de l’ART** :
- Consolidation des objectifs de programme pour l’ensemble des équipes.
- Création d’un plan global synchronisé.

6. **Engagement final** :
- Les équipes s’engagent à atteindre les objectifs du PI.
- Clôture avec une revue collective et un “mot de la fin”.

## **Livrables du PI Planning**
1. **Program Board** :
- Une visualisation des dépendances inter-équipes, des délais et des livraisons prévues.
- Indique les fonctionnalités planifiées et les dates de livraison.

2. **Objectifs d’équipe** :
- Chaque équipe définit des objectifs clairs pour le PI, incluant des objectifs métiers et techniques.

3. **Objectifs de l’ART** :
- Une vue consolidée des objectifs globaux du train Agile.

4. **Liste des risques** :
- Les risques identifiés sont documentés et, si possible, résolus durant le PI Planning.

## **Participants au PI Planning**
1. **Équipes Agile** :
- Développeurs, testeurs et autres membres techniques qui réalisent le travail.
2. **Product Owners (PO)** :
- Fournissent des détails sur les fonctionnalités et priorisent le travail.
3. **Scrum Masters** :
- Facilitent le processus pour leurs équipes.
4. **Product Manager** :
- Définit la vision et le backlog de programme.
5. **Release Train Engineer (RTE)** :
- Facilite l’événement global.
6. **Parties prenantes** :
- Inclut les sponsors, les architectes, les clients et les représentants métiers.

## **Avantages du PI Planning**
1. **Alignement complet** : Toutes les équipes partagent une vision commune et travaillent dans la même direction.
2. **Résolution proactive des dépendances** : Les risques et les obstacles sont identifiés et adressés en amont.
3. **Engagement accru** : Les équipes s’approprient leurs objectifs, ce qui renforce leur motivation.
4. **Amélioration de la collaboration** : Favorise les échanges entre équipes et départements.
5. **Visibilité stratégique** : Les parties prenantes comprennent clairement comment les initiatives stratégiques seront réalisées.

## **Pré-requis pour un PI Planning réussi**
1. **Préparation du backlog** :
- Les fonctionnalités prévues doivent être bien définies et priorisées.
2. **Calendrier clair** :
- Le PI Planning doit être intégré au calendrier global de l’ART.
3. **Outils de collaboration** :
- Utilisation d’outils comme **Miro**, **Jira**, ou **Confluence** pour faciliter la planification, surtout pour les équipes distribuées.
4. **Participation active des parties prenantes** :
- Leur présence est essentielle pour répondre aux questions et clarifier les priorités.

## **Challenges courants**
- **Trop de dépendances non résolues** : Les dépendances mal identifiées peuvent ralentir les livraisons.
- **Manque de préparation** : Un backlog mal défini ou une vision floue peut nuire à l’efficacité de l’événement.
- **Participants non engagés** : Une faible implication des équipes ou des parties prenantes peut compromettre le succès.