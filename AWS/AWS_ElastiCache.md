# What is ElastiCache

Service de base de données principalement utilisée pour du cache

# Notes

* Redis or Memcached managed
* Helps to put state of applications in cache
* Must set cache invalidation strategy
* Redis : 
    * multi AZ with auto failover
    * Read replicas have high availability
    * Data durability using AOF persistence
    * Backup and rstore features
* Memcached
    * Multi node for paritioning (sharding)
    * No high availability
    * Non persistent
    * No backup restore
* Strategies
    * Lazy loading = cache aside = lazy population
        * Read cache
        * If nothing, populate
    * Write through
        * For each write, update cache
* Time to live (TTL) and cache evictions
    * Eviction can append
        * Delete item
        * Memory is full
        * Set TTL
    * TTL not a very good idea with write through strategy
* Redis replication
    * Cluster mode disabled : 
        * 5 replicas
        * Cluster with replication
        * Primary node is read/write, others for read
        * A node is a shard
    * Cluster mode enabled
        * Data partitioned across shards
        * Each shard has a primary and up to 5 replica nodes (a replica node is a shard)
        * Multi AZ
        * Up to 500 nodes per cluster
