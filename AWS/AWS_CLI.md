# What is CLI

Command line interface s'appuyant sur Python et la librairie associée

# Notes
* In .aws, cat credentials show credentials
* To use multiple accounts, add profiles
    * Aws configure --profile <profile-name>
    * Then use aws <command> --profile <profile-name>
* To use MFA with the CLI, must create a temporary session
    * Run STS GetSessionToken : aws sts get-session-token --serial-number <mfa> --token-code <code-from-mfa-device> --duration-seconds XXX
    * Create a new profile with information received with entry aws_session_token

# AWS CLI Credentials Provider Chain
* CLI will look for credential in this order
    * Command line options region, output and profile
    * Environment variables AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY
    * CLI credentials file (.aws/credentials)
    * CLI Configuration file (.aws/config)
    * Container credentials for EC2 tasks
    * Instance profile credentials for EC2 instance profiles
* Credentials best practices
    * Use IAM roles as much as possible (when working inside AWS)
    * Use environment variables/named profiles (when working outside AWS)
