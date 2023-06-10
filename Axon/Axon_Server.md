# Axon server
Axon propose un Axon server qui contient notamment l'event store de l'architecture Event Sourcing.

L'Axon server est gratuit s'il est utilisé en monoréplica. Si l'on souhaite utiliser du sharding, il faut passer à la version payante.

## How to install
Aller sur `axoniq.io`
* As a jar
* As a Docker container `https://developer.axoniq.io/w/running-axon-server-in-docker-continuing-from-local-developer-install-to-containerized`
    * Bind volumes `eventdata` & `config` of container
    * Ports 8024 & 8124
* Exemple de docker-compose
```
version: '3.3'
services:
  axonserver:
    image: axoniq/axonserver
    hostname: axonserver
    platform: linux/arm64/v8
    volumes:
      - axonserver-data:/data
      - axonserver-events:/eventdata
      - axonserver-config:/config:ro
    ports:
      - '8024:8024'
      - '8124:8124'
      - '8224:8224'
    networks:
      - axon-demo

volumes:
  axonserver-data:
  axonserver-events:
  axonserver-config:

networks:
  axon-demo:
```

## How to configure
La liste des propriétés se trouve à `https://docs.axoniq.io/reference-guide/axon-server/administration/admin-configuration/configuration#configuration-properties`.
* Avec une installation depuis le .jar, la configuration doit aller dans un fichier placé dans un sous-dossier `config` où se trouve le jar d'Axon server. Le fichier doit s'appeler `axonserver.properties` ou `axonserver.yml`.
* Avec une installation depuis un container, utiliser des variables d'environnement
