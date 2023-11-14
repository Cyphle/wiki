# TypeORM

## What is TypeORM
* TypeORM is an ORM for NodeJS
* https://typeorm.io/
* It works with anotations such as
```
import {Entity, PrimaryGeneratedColumn, Column} from "typeorm"; 

@Entity() 
export class Student {   

   @PrimaryGeneratedColumn() 
   id: number; 
   
   @Column() 
   Name: string; 
   
   @Column() 
   age: number; 
}
```