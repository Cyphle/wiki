# GraphQL

## What is a graph
A graph is a requestable structure of data.

### Type definitions
They are schema which define what data look like. They also describe operations

### Resolvers
They resolve the schemas.

### Scalar types
* String
* Boolean
* Int
* Float
* ID
* ! in a schema means not nullable
* [] in a schema means array
* []! array not nullable but array can be empty

## Operations on a graph

### Query
When querying a graph, we have to tell which fields of an object are wanted or it won't work. It is not possible to tell "all".

