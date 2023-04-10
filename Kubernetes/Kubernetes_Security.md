# Kubernetes security

## Securing API server
* Le processus d'authentification passe par différents plugins qui permettent de déterminer les informations du user. Les mécanismes d'authentification peuvent être
    * Depuis un certificat client
    * Authentication token
    * Basic authentication
    * Autres
* L'API server permet de définir des roles et des groupes qui peuvent être attribués aux users et pods au travers de services accounts
* Pour gérer les droits, il y a le plugin RBAC (role based access control). Permet de mapper des droits comme l'utilisation de verbes HTTP

## Securing cluster nodes and network
