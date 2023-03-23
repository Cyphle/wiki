# Kubernetes

* Kubernetes utilise des Nodes qui sont des serveurs où sont lancés les ressources de travail
* Un cluster Kubernetes est composé de :
    * Worker Nodes
        * Kubelet
    * Master : manage les noeuds et orchestre les ressources
        * API server
        * etcd
        * controller
        * scheduler
* Les composants d'un cluster sont
    * API Server : endpoints pour communiquer avec le cluster
    * etcd : c'est la base de données de Kubernetes qui est clé/valeur
    * Scheduler : est chargé de distribuer les ressources dans les noeuds
    * Controller : il s'agit du cerveau de l'orchestration. Il est chargé de prendre des décisions quand les pods fail
    * Container runtime : le runtime qui permet de faire tourner les pods (exemple : Docker runtime)
    * Kubelet : l'agent à installer sur les noeuds pour parler au master
* La CLI : kubectl
* Kubernetes fonctionne à partir de descripteurs qui permettent de décrire les ressources à déployer
* Convenient tool : k9s, kubectx, kubens
* Pour minikube, le dashboard : `minikube dashboard`

## Descripteurs Kubernetes
* Les parties principales des descripteurs sont :
    * apiVersion: la version de la ressource
    * kind: le type de ressource
    * metadata: contient le nom, le namespace, les labels, les annotations et d'autres infos
        * labels: permettent d'organiser les ressources comme utiliser des selecteurs, définir des noeuds où déployer
        * annotations: permettent de contenir des informations plus longues mais ne permettent pas d'être utilisées comme les labels avec des selecteurs et autre
    * spec: la description de la ressource (par exemple les containers à lancer avec leurs ressources), containers, volumes, ressources, etc
    * status: le status de la ressource qui tourne (accessible quand elle tourne)
* Selecteurs
    * Les selecteurs permettent aux ressources d'appeler d'autres ressources. Par exemple un service va cibler des pods via des labels
    * Cela permet aussi d'autres spécification comme pour contraindre le scheduling des pods sur certains noeuds (par exemple qui ont du GPU)

## Ressources Kubernetes

### Pods
* Smallest object that can be created
* Contain one or multiple containers
* Les containers qui tournent dans le même pod partagent le même namespace Linux
* La clé `nodeSelector` dans la partie `spec` permet de définir des noeuds sur lesquels déployer (qui ont du gpu par exemple)
* liveness : sonde à définir afin d'indiquer quand un pod est up
* readiness : sonde à définir afin d'indiquer quand un pod est prêt à recevoir du traffic (en général il s'agit un health transitif)
* Dans la partie `spec` il est important de définir des ressources qui permettent de bien scheduler les pods sur les noeuds en fonction des requests et limites

### Services
* Il s'agit des éléments qui permettent d'avoir une IP fixe vers les pods
* Etant donné que les pods peuvent être détruits et recréés n'importe où, ils changent d'IP
* Avec les services, les pods peuvent se trouver via service name
* Un service peut définir des sessions affinity
* La définition des ports peut utiliser les noms des ports des pods
* Avec les services, il est possible d'exposer des services externes à l'intérieur du cluster
    * Définir une ressource endpoint qui pointe vers le service externe
    * La rattacher à un service


### Replication controllers
* Ressource qui s'assure que les pods surveillés sont toujours up.
* Permet de gérer plusieurs instances de pods et conserve up le nombre de replicas spécifiés
* C'est une ancienne version de replica set

### ReplicaSets
* Se comporte comme un replication controller
* Un replica set peut utiliser des label selectors où une ressource n'a pas le label alors qu'un replication controller ne peut sélectionner que des ressources qui ont le label désiré
* Il est possible de sélectionner uniquement par clé également
* Peut utiliser des matchExpressions pour les sélecteurs
    * In
    * NotIn
    * Exists
    * DoesNotExist

### DeamonSets
* Run one pod on each node
* Possible de ne lancer les pods que sur certains noeuds avec les node selector

### Job resource
* Run one task then kill the pod
* Il est possible de créer des job qui doivent se lancer plusieurs fois via des `multi-completion-batch-job`

### CronJob
* Pareil que les job mais schedulés

### Deployments

### StatefullSets

### ConfigMaps

### Secrets

### Network policies

### PersistentVolume et PersistentVolumeClaim

### Namespace
* Un namespace Kubernetes permet de ségréger des ressources et de les grouper
* Attention, par défaut les namespaces permettent de grouper des ressources mais n'isole pas les namespaces entre eux.


## Commandes Kubectl
### Context
* `kubectl config view` : liste les config
* `kubectl config current-context` : affiche le contexte courant
* `kubectl config use-context <context>` : set le context courant à <context>

### Cluster
* `kubectl cluster-info` : informations sur le cluster
* `kubectl get nodes` : liste des noeuds

### Autres
* `kubectl run <descripteur>` : run un pod
* `kubectl describe <object_type> <object_id>` : décrit <object_type> portant l'id <object_id>
* `kubectl get <object_type>` : liste les ressources de type <object_type>
* `kubectl explain <object_type>` : affiche une description de l'<object_type>
* `kubectl apply <descripteur>` : déploie une ressource (déclaratif)
* `kubectl create <descripteur>` : créé la ressource (imperatif)
* `kubectl delete <object_type> <object_id> <options>` : delete la ressource. Options :
    * --cascade=false : par exemple pour un replication controller ne delete pas les pods
* `kubectl exec <pod> -- curl -s <something>` : éxecuter ce qui se trouve après '--' dans le pod