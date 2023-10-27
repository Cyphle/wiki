# GraphQL Resolvers

## What are resolvers
* Resolvers are functions in charge to populate data of single fields of a schema
* Example
```
type Query {
  numberSix: Int! # Should always return the number 6 when queried
  numberSeven: Int! # Should always return 7
}

const resolvers = {
  Query: {
    numberSix() {
      return 6;
    },
    numberSeven() {
      return 7;
    },
  },
};
```
* A resolver can optionally have 4 positional arguments
```
const resolvers = {
  Query: {
    user(parent, args, contextValue, info) {
      return users.find((user) => user.id === args.id);
    },
  },
};

even if schema is
type User {
  id: ID!
  name: String
}

type Query {
  user(id: ID!): User
}
```
`args` argument contains arguments provided for the field
* Note that if field are obvious to resolve, it is not needed to define a resolver as there will be one be default (getter of object for instance)