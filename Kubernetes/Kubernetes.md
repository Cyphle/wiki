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
* Probes à définir
    * Readiness
    * Liveness

## Ressources Kubernetes

### Pods
* Smallest object that can be created
* Contain one or multiple containers
    * A noter que :
        * `ENTRYPOINT ["node", "app.js"]`: lance le process directement
        * `ENTRYPOINT node app.js`: lance le process dans un bash
* Les containers qui tournent dans le même pod partagent le même namespace Linux
* La clé `nodeSelector` dans la partie `spec` permet de définir des noeuds sur lesquels déployer (qui ont du gpu par exemple)
* liveness : sonde à définir afin d'indiquer quand un pod est up
* readiness : sonde à définir afin d'indiquer quand un pod est prêt à recevoir du traffic (en général il s'agit un health transitif)
* Dans la partie `spec` il est important de définir des ressources qui permettent de bien scheduler les pods sur les noeuds en fonction des requests et limites

### Services
* Il s'agit des éléments qui permettent d'avoir une IP fixe vers les pods
* Pour exposer le service uniquement à l'intérieur du cluster il faut mettre son type en ClusterIP (type par défaut)
* Etant donné que les pods peuvent être détruits et recréés n'importe où, ils changent d'IP
* Avec les services, les pods peuvent se trouver via service name
* Un service peut définir des sessions affinity
* La définition des ports peut utiliser les noms des ports des pods
* Avec les services, il est possible d'exposer des services externes à l'intérieur du cluster
    * Définir une ressource endpoint qui pointe vers le service externe
    * La rattacher à un service
* Exposer des services à l'extérieur du cluster
    * En définissant le type du service en NodePort : expose un port du noeud à l'extérieur
    * En définissant le type du service en LoadBalancer : expose via un load balancer qui va router le traffic vers les services des différents noeuds
    * En créant un Ingress

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

### Ingress
* Ressource permettant de router du traffic vers des services
* Se base sur le host et le path
* Ressource au niveau 7 de la couche OSI
* Permet d'appliquer des cookie based sessions affinity
* Pour que les ingress marchent, il faut un ingress controleur
    * NGinx
    * Cloud load balancer
* Pour le SSL/TLS, il n'y a besoin qu'entre le client et l'ingress controler
    * Attach certificate and private key to ingress as secret

### Volumes
* Used to attach disk storage to containers
* Créé lors de la création d'un pod et détruit à la destruction du pod
* Volume types
    * emptyDir : simple directory for transient data
    * hostPath : for mounting directories from worker node filesystem
    * gitRepo : initialized by checking out the content of a git repository
    * nfs : nfs share mount
    * gcePersistentDisk : for Google persistent disk
    * awsElasticBlockStore : for AWS EBS
    * azureDisk : for Microsoft
    * cinder, cephfs, iscsi, flocker, glusterfs, quobyte, rbd, flexVolume, vsphere-Volume, photonPeristentDisk, scaleIO : other types of network storage
    * configMap, secret, downwardAPI
    * persistentVolumeClaim : use a dynamically provisioned persistent storage

### PersistentVolumes & PersistentVolumeClaims
* Permet de monter des volumes persistent sans en connaitre la nature
* PersistentVolumes permettent de créer des ressources volume qui elles connaissent le type de stockage derrière (EBS, etc)
* PersistentVolumeClaims permettent à un pod d'être bindé à un PersistentVolume
* Un seul PersistentVolume peut être bindé à un PersistentVolumeClaim. Pour libérer le volume, il faut delete le pvc
* Les pv sont des ressources au niveau cluster et non namespace
* Au lieu de définir des pv manuellement, il est possible de créer un pv provisioner qui définit des storage class (les cloud providers en proposent en général)

### StorageClass
* Permettent de provisioner automatiquement des pv en proposant des types de volume
* Les pvc définissent alors un storage class à utiliser pour que le pv soit créé

### Deployments

### StatefullSets

### ConfigMaps

### Secrets

### Network policies

### PersistentVolume et PersistentVolumeClaim

### Namespace
* Un namespace Kubernetes permet de ségréger des ressources et de les grouper
* Attention, par défaut les namespaces permettent de grouper des ressources mais n'isole pas les namespaces entre eux.

### Sidecars
* Un sidecar est un container qui va s'ajouter au container principal pour ajouter des features


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
* `kubectl apply <descripteur>` : déploie une ressource (déclaratif). S'il y a un update de ressource, utiliser apply
* `kubectl create <descripteur>` : créé la ressource (imperatif)
* `kubectl delete <object_type> <object_id> <options>` : delete la ressource. Options :
    * --cascade=false : par exemple pour un replication controller ne delete pas les pods
* `kubectl exec <pod> -- curl -s <something>` : éxecuter ce qui se trouve après '--' dans le pod