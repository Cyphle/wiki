# Prisma ORM

## What is Prisma
* Prisma is an ORM for NodeJS applications
* Prisma has some plugins like Prisma Bindings to bind GraphQL types with Prisma Model (https://www.npmjs.com/package/prisma-binding)
* Prisma has usefull tools such as migration and studio (GUI to explore database)
* https://www.prisma.io/typescript

## Basic principles
* Prisma works with two main components: Prisma CLI and Prisma client
* Use Prisma CLI to connect to database and run operations such as migrations
```
prisma db pull 

prisma migrate dev --name init
```
* For setup and defining schema, use a `schema.prisma` file
```
// This is your Prisma schema file,
// learn more about it in the docs: https://pris.ly/d/prisma-schema

generator client {
  provider = "prisma-client-js"
}

datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
}

// For spltting in different files: https://github.com/ajmnz/prisma-import ou https://github.com/zenstackhq/zenstack

model User {
  id    Int     @id @default(autoincrement())
  email String  @unique
  name  String?
  posts Post[]
}

model Post {
  id        Int     @id @default(autoincrement())
  title     String
  content   String?
  published Boolean @default(false)
  author    User    @relation(fields: [authorId], references: [id])
  authorId  Int
}
```
* Use Prisma client in code
```
const prisma = new PrismaClient();

async function main() {
  const user = await prisma.user.create({
    data: {
      name: 'Alice',
      email: 'alice@prisma.io',
    },
  })
  console.log(user)
}

main()
      .then(async () => {
        await prisma.$disconnect()
      })
      .catch(async (e) => {
        console.error(e)
        await prisma.$disconnect()
        process.exit(1)
      })
```

## Notes
* Prisma uses the file schema.prisma to setup database connection and schema definitions. It does NOT support multiple schema files. For this, use ZenStack (https://github.com/zenstackhq/zenstack) which is an extention to prisma CLI
* Prisma migration tool is REALLY limited. Like it is not possible to rename a column as it is based on reading the final state of the database model. If a column is renamed for example, Prisma will drop the old one and create the new one. To overcome this limitation, use other tools like Knex.js (https://knexjs.org/guide/migrations.html#migration-cli)