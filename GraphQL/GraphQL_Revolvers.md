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

## Notes
* When resolving subtypes, types define the relation and resolvers have to define how to resolve the relation
```
const users = [
  {
    id: '1',
    name: 'Andrew',
    email: 'andrew@example.com',
    age: 27
  },
  {
    id: '2',
    name: 'Sarah',
    email: 'sarah@example.com'
  },
  {
    id: '3',
    name: 'Mike',
    email: 'mike@example.com'
  }
];

const posts = [
  {
    id: '10',
    title: 'GraphQL 101',
    body: 'This is how to use GraphQL...',
    published: true,
    author: '1'
  },
  {
    id: '11',
    title: 'GraphQL 201',
    body: 'This is an advanced GraphQL post...',
    published: false,
    author: '1'
  },
  {
    id: '12',
    title: 'Programming Music',
    body: '',
    published: false,
    author: '2'
  }
];

const typeDefs = `
  type Query {
    users(query: String): [User!]!
    posts(query: String): [Post!]!
    me: User!
    post: Post!
  }
  
  type User {
    id: ID!
    name: String!
    email: String!
    age: Int
  }
  
  type Post {
    id: ID!
    title: String!
    body: String!
    published: Boolean!
    author: User!
  }
`;

const resolvers = {
  Query: {
    users(parent: any, args: any, ctx: any, info: any) {
      if (!args.query) {
        return users;
      }

      return users.filter((user: any) => {
        return user.name.toLowerCase().includes(args.query.toLowerCase());
      });
    },
    posts(parent: any, args: any, ctx: any, info: any) {
      if (!args.query) {
        return posts;
      }

      return posts.filter((post: any) => {
        const isTitleMatch = post.title.toLowerCase().includes(args.query.toLowerCase());
        const isBodyMatch = post.body.toLowerCase().includes(args.query.toLowerCase());
        return isTitleMatch || isBodyMatch;
      });
    }
  },
  Post: {
    author(parent: any, args: any, ctx: any, info: any) {
      return users.find((user: any) => {
        return user.id === parent.author;
      });
    }
  }
};
``` 