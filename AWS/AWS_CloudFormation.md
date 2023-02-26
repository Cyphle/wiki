# What is CloudFormation ?

* Concurrent à Terraform mais spécifique à AWS
* IaC (Infra as code) pour AWS
* Permet de définir l'infra AWS et sa config à partir de code

# Introduction

* Is a declarative way of outlniing AWS infra for any resources
* Each resources has an identifier to identify its cost
* Can estimate cost using CloudFormation template
* Can automate deletion/creation of templates at given time for example
* Can generate diagram and doc
* Templates are uploaded in S3
* Can't edit a template, have to upoad a new version
* Deploying templates
    * Manuel
        * Edit in CloudFormation designer
        * Use console to input parameters
    * Automated
        * Edit templates in yaml
        * Use CLI to deploy
* Blocks
    * Resources (mandatory)
    * Parameters
    * Mappings
    * Outputs
    * Conditionals
    * Metadata
* Template helpers
    * References
    * Functions

# Example template

## One instance
```
---
Resources:
    MyInstance:
        Type: AWS::EC2::Instance
        Properties:
            AvailabilityZone: us-east-1a
            ImageId: ami-a4c7edb2
            InstanceType: t2.micro

    MyIP:
        Type: AWS::EC2::EIP
        Properties: 
            Domain: 123.456.789.123
            InstanceId: !Ref MyInstance
```

# Resources

* Represents AWS components to create and configure
* Can reference each other
* Identifiers are : AWS::aws-product-name::data-type-name
* Doc: search for AWS Resource Type Reference
* Cannot create dynamic amount of resource
* Some services are not supported

# Parameters

* Provide inputs to template
* To use a reference to a parameter use
```
!Ref <parameter>
```
* Pseudo parameters for example AWS::AccountId is a parameters existing without declaring it (there are others)

# Mappings

* Are fixed variables in templates
* Usefull to differentiate like environments, regions, AMI types, etc
* Example
```
RegionMap:
    us-east-1:
        "32": "ami-id-1"
        "64": "ami-id-2"
    us-west-2:
        ...
```
* To find in map
```
!FindInMap [MapName, TopLevelKey, SecondLevelKey]
!FindInMap [RegionMap, !Ref "AWS::Region", 32]
```

# Outputs

* Export values to import them in other templates
* Can print in console or cli
* Cannot delete a CloudFormation stack if outputs are reference in another stack
```
Outputs:
    StackSSHSecurityGroup:
        Description: My description
        Value: !Ref MyCompanySSHSG
        Export:
            Name: SSHSecurityGroup
```
To import
```
!ImportValue <Output_Name>
```
* Outputs must have unique name within a region

# Conditions

* Conditions can reference each others
```
Conditions:
    CreateProdResources: !Equals [ !Ref EnvType, prod ]
```
* Functions
    * And
    * Equals
    * If
    * Not
    * Or
* To use
```
Resources:
    MountPoint:
        Type: "AWS::EC2::VolumeAttachment"
        Condition: CreateProdResources
```

# Intrisic Functions

* Use function without 'Fn::' but with '!' like '!Ref'
* Fn::Ref
    * Reference a parameter then returns value of parameter
    * Reference resource then returns physical ID of resource
```
!Ref <stuffs>
```
* Fn::GetAtt
    * Attributes are attached to resources
    * Know attributes of resources
```
Resources:
    EC2Instance:
        Type: "AWS::EC2::Instance"
        Properties:
            ImageId: ami-xxx
            InstanceType: t2.micro
    NewVolume:
        Type: "AWS::EC2::Volume""
        Condition: CreateProdResources
        Properties:
            Size: 100
            AvailabilityZone:
                !GetAtt EC2Instance.AvailabilityZone
```
* Fn::FindInMap
    * Return value of a mapping
* Fn::ImportValue
    * Import values that are exported from other templates
* Fn::Join
    * Join values with a delimiter
```
!Join [ ":", [a, b, c ] ] => a:b:c
```
* Fn::Sub
    * Substitute value with string (substitute values)
```
!Sub
    - String
    - { Var1Name: Var&Value, Var2Name: Var2Value }
```

# Rollbacks

* When creating: By default everything is rollback if stack fails to be created
* When update: if fails, rollback to previous state
* Can change options to not rollback all stack but only failed components

# ChangeSets

* Know in advance what will happen
* Allow to view changes before it happens
* ChangeSets won't say if update will be successfull

# Nested stacks

* Stacks that are parts of other stacks
* Allow isolate repeated patterns/common components in separate stack and call them from other stacks
* To update nested stack, update parent

## Cross vs Nested stacks

* Cross stacks
    * When stacks have different lifecycle
    * Use outputs and import
    * Example: when need to pass export values to many stacks like VPC id
* Nested stacks
    * When components must be re-used

# StackSets

* Create, update, delete stacks accross multiple accounts and regions
* Administrator account can create stack sets
* Trusted accounts to launch create update and delete
* If you update a stackset, all stacks are updates

# CloudFormation Drift

* CloudFormation does not protect agains manuel configuration changes (someone can do that)
* Theses are called drift because they drift from templates
* Use CloudFormation drift to detect them (beware, not resources support it yet)