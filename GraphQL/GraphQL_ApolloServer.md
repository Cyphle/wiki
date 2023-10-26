# Apollo Server

## Introduction
* GraphQL compliant server
* Can be used for graphql federation (supergraph)
* Can be used with NodeJS applications, AWS lambda, Azure function, Cloudflare
* Apollo Server can fetch data from any source you connect to (including a database, a REST API, a static object storage service, or even another GraphQL server)

## Notes
* Context passed to `new ApolloWerver` is used to type the resolvers and ensure that all types used are known.
```
interface MyContext {
  token?: String;
  dataSources: {
    books: Book[];
  };
}

const server = new ApolloServer<MyContext>({
  typeDefs,
  resolvers,
  plugins: [ApolloServerPluginDrainHttpServer({ httpServer })],
});
```