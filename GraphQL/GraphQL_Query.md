# GraphQL Query

## Notes
* When querying a graph, we have to tell which fields of an object are wanted or it won't work. It is not possible to tell "all".
* Is a special object that defines all of the top level entry points for queries that clients execute
* It is possible to query multiple entry points. For example
```
Schema:
type Query {
  books: [Book]
  authors: [Author]
}

Query:
query GetBooksAndAuthors {
  books {
    title
  }

  authors {
    name
  }
}

Result:
{
  "data": {
    "books": [
      {
        "title": "City of Glass"
      },
      ...
    ],
    "authors": [
      {
        "name": "Paul Auster"
      },
      ...
    ]
  }
}
```