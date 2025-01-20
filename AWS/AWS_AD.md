# AD

# What is AD ?

AD, Microsoft Active Directory, is the service with the same name and hosted on AWS.

An AD can run on Windows Server with AD Domain Services. It provides a database of objects for users, accounts, computers, printers, file shares and security groups.

Objects are organized in trees and a group of trees is a forest.

The domain controller checks for permisssions.

AWS Directory Services allows to create or use a Microsoft AD with the following configurations:
- AWS Managed Microsoft AD
	- The AD is hosted on AWS and it supports MFA
	- It can establish a trust connection with on-premise AD
- AD Connector
	- It is a Directory Gateway (proxy) to redirect to on-premise AD and it supports MFA
- Simple AD
	- It is an AD compatible managed directory
	- It cannot be joined with on-premise AD
