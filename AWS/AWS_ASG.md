# What is ASG

Pour Auto sclaing group

Resource permettant de scaler automatiquement d'autres ressources comme les EC2 en fonction de critères

# Notes
* Automatic registering to ELB
* Are free. Pay only for underlying resources
* Can terminate instances if ELB see unhealthy
* Need to create Launch configurations : how to launch EC2
* Can be based on cloud watch alarms
* Dynamic scaling policies
    * Target tracking scaling : track for exemple average CPU
    * Simple/Step scaling : when a cloudwatch alarm is traggered (exemple CPU > 70%)
    * Scheduled actions : anticipate a scaling based on know usage (exemple scale every Friday)
    * Predictive scaling : continuously forecast and scale ahead
* Good Metrics to scale on
    * CPU utilisation
    * Request count per target
    * Average network
    * Any custom metric
* After scaling activity enter cooldown period
    * ASG will not launch or terminate additional instance to allow stabilization of metrics
