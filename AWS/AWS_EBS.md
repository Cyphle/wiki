# What is EBS

Pour Elastic Block Store.

Il s'agit d'une solution de stockage qui peuvent être liés à des EC2.

# Notes
* Is a network drive
* Persist data
* AZ specific
* Delete on termination : pour les EBS créé avec une instance
* Pas obligé de détacher l'EBS pour un snapshot mais recommandé
* Snapshot peut être fait cross AZ
* Snapshot archive : takes 24 to 72 to restore
* Fastsnapshot : really fast to recover but expensive
* Types
    * Gp2/gp3 : general purpose SSD (1gb to 16tb). Gp3 can have more iops. Can set iops in gp3
    * Io1/io2 : highest performance SSD
    * St1 : low cost hdd
    * Sc1 : lowest cost hdd
    * Only gp2/gp3 and io1/io2 can be used for booting
* Provisionned iops
    * Io1/io2 : nitro ec2 have more iops
* Multi Attach
    * Attach same EBS to multiple EC2 in same AZ for io1/io2 (max 16 EC2)
		