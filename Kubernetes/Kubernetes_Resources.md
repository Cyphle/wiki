# Kubernetes Resources

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
* Pour récupérer des images depuis un repository privé, il faut utiliser des ImagePullPolicy

## Pods
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

## Services
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

## Replication controllers
* Ressource qui s'assure que les pods surveillés sont toujours up.
* Permet de gérer plusieurs instances de pods et conserve up le nombre de replicas spécifiés
* C'est une ancienne version de replica set

## ReplicaSets
* Se comporte comme un replication controller
* Un replica set peut utiliser des label selectors où une ressource n'a pas le label alors qu'un replication controller ne peut sélectionner que des ressources qui ont le label désiré
* Il est possible de sélectionner uniquement par clé également
* Peut utiliser des matchExpressions pour les sélecteurs
    * In
    * NotIn
    * Exists
    * DoesNotExist

## DeamonSets
* Run one pod on each node
* Possible de ne lancer les pods que sur certains noeuds avec les node selector

## Job resource
* Run one task then kill the pod
* Il est possible de créer des job qui doivent se lancer plusieurs fois via des `multi-completion-batch-job`

## CronJob
* Pareil que les job mais schedulés

## Ingress
* Ressource permettant de router du traffic vers des services
* Se base sur le host et le path
* Ressource au niveau 7 de la couche OSI
* Permet d'appliquer des cookie based sessions affinity
* Pour que les ingress marchent, il faut un ingress controleur
    * NGinx
    * Cloud load balancer
* Pour le SSL/TLS, il n'y a besoin qu'entre le client et l'ingress controler
    * Attach certificate and private key to ingress as secret

## Volumes
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

## PersistentVolumes & PersistentVolumeClaims
* Permet de monter des volumes persistent sans en connaitre la nature
* PersistentVolumes permettent de créer des ressources volume qui elles connaissent le type de stockage derrière (EBS, etc)
* PersistentVolumeClaims permettent à un pod d'être bindé à un PersistentVolume
* Un seul PersistentVolume peut être bindé à un PersistentVolumeClaim. Pour libérer le volume, il faut delete le pvc
* Les pv sont des ressources au niveau cluster et non namespace
* Au lieu de définir des pv manuellement, il est possible de créer un pv provisioner qui définit des storage class (les cloud providers en proposent en général)

## StorageClass
* Permettent de provisioner automatiquement des pv en proposant des types de volume
* Les pvc définissent alors un storage class à utiliser pour que le pv soit créé

## ConfigMaps
* Les configmaps permettent d'exposer dans le pod des informations comme des variables d'environnements et des fichiers
* De base pour injecter des variables d'environnements dans un pods, il est possible de définir dans les specs
```
kind: Pod
spec:
    containers:
    - image: luksa/fortune:v1
      env:
      - name: INTERVAL
        value: "30"
      - name: FIRST_VAR
        value: "foo"
      - name: SECOND_VAR
        value: "$(FIRST_VAR)bar"
```
* Mais il est préférable d'utiliser des configmaps. Les pods utilisent les utilisent via des variables d'environnements et les configmaps sont dans des volumes
* Il est possible de créer des configmaps à partir d'un fichier `kubectl create configmap my-config --from-file=config-file.conf`
* Il est possible de créer des configmaps à partir de tous les fichiers d'un dossier `kubectl create configmap my-config --from-file=/path/to/dir`
* Pour passer une variable d'une configmap à un pod
```
apiVersion: v1
kind: ConfigMap
data:
    sleep-interval: "25"
metadata:
    creationTimestamp: 2016-08-11T20:31:08Z
    name: fortune-config
    namespace: default
    resourceVersion: "910025"
    selfLink: /api/v1/namespaces/default/configmaps/fortune-config
    uid: xxx

------
# Une variable
apiVersion: v1
kind: Pod
metadata:
    name: fortune-env-from-configmap
spec:
    containers:
    - image: luksa/fortune:version
      env:
      - name: INTERVAL
        valueFrom:
            configMapKeyRef:
                name: fortune-config
                key: sleep-interval

------
# Toute la configmap
apiVersion: v1
kind: Pod
metadata:
    name: fortune-env-from-configmap
spec:
    containers:
    - image: luksa/fortune:version
      envFrom:
      - prefix: CONFIG_
        configMapRef:
            name: my-config-map
```
* Pour les variables plus importantes, il est possible de passer des fichiers entiers
```
apiVersion: v1
kind: Pod
metadata:
    name: fortune-configmap-volume
spec:
    containers:
    - image: nginx:alpine
      name: web-server
      volumeMounts:
      ...
      - name: config
        mountPath: /etc/nginx/conf.d
        readOnly: true
      ...
    volumes:
    ...
    - name: config
      configMap:
        name: fortune-config
    ...
```

## Secrets
* Pour passer des informations sensible, il est préférable de passer par des secrets. Passage par variables d'environnements ou volumes
* Chaque pod à un default token Secret
* Les secrets définissent deux entrées: pour les valeurs plain text et celles base64
```
kind: Secret
apiVersion: v1
stringData:
    foo: plain text
data:
    https.cert: ...

-------
apiVersion: v1
kind: Pod
metadata:
    name: fortune-https
spec:
    containers:
    - image: luksa/fortune
      name: html-generator
      env:
      - name: INTERVAL
        valueFrom:
            configMapKeyRef:
                name: fortune-config
                key: sleep-interval
      - name: FOO_SECRET
        valueFrom:
            secretKeyRef:
                name: fortune-https
                key: foo
      volumeMounts:
      - name: html
        mountPath: /var/htdocs
    - image: nginx:alpine
      name: web-server
      volumeMounts:
      - name: html
        mountPath: /usr/share/nginx/html
        readOnly: true
      - name: config
        mountPath: /etc/nginx/conf.d
        readOnly: true
      - name: certs
        mountPath: /etc/nginx/certs
        readOnly: true
      ports:
      - containerPort: 80
      - containerPort: 443
    volumes:
    - name: html
      emptyDir: {}
    - name: config
      configMap:
        name: fortune-config
        items:
        - key: my-nginx-config.conf
          path: https.conf
    - name: certs
      secret:
        secretName: fortune-https
```
* Il est possible de récupérer des secrets depuis du distant
```
# Créer un secret docker-registry nommé mydockerhubsecret
kubectl create secret docker-registry mydockerhubsecret --docker-username=myusername --docker-password=mypassword --docker-email=my.email@mail.com

------
apiVersion: v1
kind: Pod
metadata:
    name: private-pod
spec:
    imagePullSecrets:
    - name: mydockerhubsecret
    containers:
    - image: username/private:tag
      name: main
```

## Deployments
* Les deployments permettent de gérer la mise à jour des pods avec des stratégies comme rolling updat
* Il s'agit des resources à privilégier par rapport au replication controller par exemple

## StatefullSets
* Cette ressource permet de créer des pods qui ont un état comme les bases de données
* Ressource utilisée notamment lorsqu'il y a besoin de PV et PVC ou de volumes genre EBS
* Les statefullsets ne seront pas déplacés par Kubernetes lors d'équilibres de charge

## Network policies
* Permettent de restreindre quelle ressource a le droit

## Namespace
* Un namespace Kubernetes permet de ségréger des ressources et de les grouper
* Attention, par défaut les namespaces permettent de grouper des ressources mais n'isole pas les namespaces entre eux.

## Sidecars
* Un sidecar est un container qui va s'ajouter au container principal pour ajouter des features

## Ambassador
* Il s'agit d'un pattern où un container est déployé dans le même pod qu'un autre container et qui permet de faire proxy pour parler à l'API server