# Zero Downtime Deployment

* One solution is to expose the AMI in input because web server definition might be inside AMI.
* Take advantage of `create_before_destroy`setting set on instance
    1. Configure `name`parameter of ASG to depend on name of the launch configuration
    2. Set `create_before_destroy`parameter of the ASG to true
    3. Set `min_elb_capacity` to the ASG `min_size`

## Limitations
* Does not work with auto scaling policies. Resets ASG size to min_size after each ployment. Par exemple, si on a une `aws_autoscaling_schedule`qui set le nombre de réplicas à 10 à 9h et qu'on lance un deploiement à 9h avec un `min_size` à 2, cela va créer 2 replicas et attendre le lendemain pour mettre à 10.
* Pour éviter cela, il est possible d'utiliser une solution AWS `instance refresh`
