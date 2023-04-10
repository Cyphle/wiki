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
* Il est possible d'accéder à l'API Kubernetes depuis les pods. Il est ainsi possible d'accéder à des informations telles qui le CPU et la mémoire, les annotations, etc.
```
env:
- name: POD_NAME
  valueFrom:
    fieldRef:
      fielPath: metadata.name
```
* Pour accéder à l'API Kubenetes depuis la CLI, utiliser `kubectl proxy`

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