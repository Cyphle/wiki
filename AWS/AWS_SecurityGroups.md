# What are security groups

Ce sont des ressources qui permettent d'appliquer de la sécurité sur d'autres ressources (EC2, ELB, ...).

Ils définissent les règles de traffic entrant et traffic sortant.

# Notes
* Contain only allow
* Can reference each other
* Rule ex : group attaché à EC2 : quelle ip a le droit de lui parler et qu'est ce qui a le droit de sortir
* C'est stateful : c'est-à-dire qu'une requête qui sort aura le droit de rentrer
* C'est au niveau région
* Par défaut all inbound blocked et all outbound authorized
* Référence SG to SG
    * Si EC2 1 allows SG2 and SG1
    * Si EC2 2 has SG2
    * Then EC2 1 allows EC2 2 for inbound
* Statefull : si une requête peut entrer, elle peut sortir