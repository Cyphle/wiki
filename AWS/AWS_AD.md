# What is AD ?

For Microsoft Active Directory

# Notes
* On any Windows Server with AD Domain services
    * Database of objects for users, accounts, computers, printers, file shares, security groups
    * Objects are organized in trees
    * A group of tree is a forest
* Domain controller check for permissions
* AWS Directory Services allow to create Microsoft AD
    * AWS Managed Microsoft AD
        * Own AD in AWS: supports MFA
        * Can establish a trust connection with on-premise AD
    * AD Connector
        * Directory Gateway (proxy) to redirect on-premise AD: supports MFA
    * Simple aD
        * AD-compatible managed directory
        * Cannot joined with on-premise AD
        