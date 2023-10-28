# GraphQL Mutation

## Notes
* Mutation type defines endpoints to write
* Example
```
type Mutation {
  addBook(title: String, author: String): Book
}

Usage:
mutation CreateBook {
  addBook(title: "Fox in Socks", author: "Dr. Seuss") {
    title
    author {
      name
    }
  }
}
```
* It is recommended to send back data including the one(s) that have been created/modified in the response
* It is recommanded to have a mutation response interface that the returned objects implement
```
interface MutationResponse {
  code: String!
  success: Boolean!
  message: String!
}

type UpdateUserEmailMutationResponse implements MutationResponse {
  code: String!
  success: Boolean!
  message: String!
  user: User
}
```