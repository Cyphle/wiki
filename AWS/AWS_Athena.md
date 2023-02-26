# What is Athena ?

It is a serverless query service to analyze data stored in S3

# Notes
* Uses standard SQL to query the files
* Supports CSV, JSON, ORC, Avro and Parquet
* Princing: 5$ per TB of data scanned
* Commonly used with Amazon Quicksight for reporting and dashboards
* Use cases
    * BI
    * Analytics
    * Reporting
    * Flow logs
    * etc
* Use columnar data for cost saving
    * Use Apache Parquet or ORC
    * Improve performance
    * Use Glue to convert to Parquet or ORC
* Support compressed data (bzip2, gzip, ...)
* Partition dataset reflect S3 folders
* Use bigger files (> 128Mb) in S3 to minimize overhead (they are easier to scan and retrieve) for perf optim
* Federated Query
    * Allows to query data outside like database (SQL and NoSQL) in AWS or on premise
    * Use Data Source Connectors that run on lambda
