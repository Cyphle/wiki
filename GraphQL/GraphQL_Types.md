# GraphQL Types

## Type definitions/Schemas
Schemas define what data look like. They also describe operations query and mutation

## Type definition categories
* Scalar
* Object (including special root operation types Query, Mutation and Subscription)
* Input
* Enum
* Union
* Interface

### Scalar types
* String
* Boolean
* Int
* Float
* ID
* ! in a schema means not nullable
* [] in a schema means array
* []! array not nullable but array can be empty
* [item!] means that item inside array cannot be null

### Object type
* On object type can be filled by multiple data sources

### Query
* A query is a type of GraphQL
* [Query](https://github.com/Cyphle/wiki/blob/main/GraphQL/GraphQL_Query.md)

### Mutation
* A mutation is a type of GraphQL
* [Mutation](https://github.com/Cyphle/wiki/blob/main/GraphQL/GraphQL_Mutation.md)

### Input
* Input types are used to define structure that can be passed inside endpoints of query and mutations
* Example
```
input BlogPostContent {
  title: String
  body: String
}

type Mutation {
  createBlogPost(content: BlogPostContent!): Post
  updateBlogPost(id: ID!, content: BlogPostContent!): Post
}
```
* Ils peuvent être utiliser par exemple pour appliquer des filtres de recherche

### Enum
* Similar as scalar but limited to values defined in enum
* Enum values are serialized as string
* Example:
```
enum AllowedColor {
  BLUE
  GREEN
}

type Query {
  favoriteColor: AllowedColor # enum return value
  avatar(borderColor: AllowedColor): String # enum argument
}

Usage:
query GetAvatar {
  avatar(borderColor: RED)
}
```

### Union
* Used to merge multiple types (it is one of the type)
* `union Media = Book | Movie` means book OR movie
* Usage:
```
query GetSearchResults {
  search(contains: "Shakespeare") {
    # Querying for __typename is almost always recommended,
    # but it's even more important when querying a field that
    # might return one of multiple types.
    __typename
    ... on Book {
      title
    }
    ... on Author {
      name
    }
  }
}
```
* `...` is an inline fragment. It is a concept to access the underlying types
* To resolve an union, have to use `__resolveType`
```
const resolvers = {
  SearchResult: {
    __resolveType(obj, contextValue, info){
      // Only Author has a name field
      if(obj.name){
        return 'Author';
      }
      // Only Book has a title field
      if(obj.title){
        return 'Book';
      }
      return null; // GraphQLError is thrown
    },
  },
  Query: {
    search: () => { ... }
  },
};
```

### Interface
* An interface is an abstract type that other types can implement. Types that implement an interface must include all fields
```
interface Book {
  title: String!
  author: Author!
}

type Textbook implements Book {
  title: String! # Must be present
  author: Author! # Must be present
  courses: [Course!]!
}
```
* It is possible to query fields of interface and subfields of implementations but have to use a resolver like:
```
query GetBooks {
  books {
    # Querying for __typename is almost always recommended,
    # but it's even more important when querying a field that
    # might return one of multiple types.
    __typename
    title
    ... on Textbook {
      courses {
        # Only present in Textbook
        name
      }
    }
    ... on ColoringBook {
      colors # Only present in ColoringBook
    }
  }
}
```

### Custom scalars
* Need to define
    * How the scalar's value is represented in your backend
    * How the value's back-end representation is serialized to a JSON-compatible type
    * How the JSON-compatible representation is deserialized to the back-end representation
* Example of Date scalar
```
import { GraphQLScalarType, Kind } from 'graphql';

const dateScalar = new GraphQLScalarType({
  name: 'Date',
  description: 'Date custom scalar type',
  serialize(value) {
    if (value instanceof Date) {
      return value.getTime(); // Convert outgoing Date to integer for JSON
    }
    throw Error('GraphQL Date Scalar serializer expected a `Date` object');
  },
  parseValue(value) {
    if (typeof value === 'number') {
      return new Date(value); // Convert incoming integer to Date
    }
    throw new Error('GraphQL Date Scalar parser expected a `number`');
  },
  parseLiteral(ast) {
    if (ast.kind === Kind.INT) {
      // Convert hard-coded AST string to integer and then to Date
      return new Date(parseInt(ast.value, 10));
    }
    // Invalid hard-coded value (not an integer)
    return null;
  },
});

const typeDefs = `#graphql
  scalar Date

  type Event {
    id: ID!
    date: Date!
  }

  type Query {
    events: [Event!]
  }
`;

const resolvers = {
  Date: dateScalar,
  // ...other resolver definitions...
};
```

## Directives
* A directive is a decorator that can allow tools like Apollo Server to perform custom logic
* Directives can only be applied where it is meant to be