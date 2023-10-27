# Apollo Server

## Introduction
* https://www.apollographql.com/docs/apollo-server/
* https://www.apollographql.com/tutorials/browse
* GraphQL compliant server
* Can be used for graphql federation (supergraph)
* Can be used with NodeJS applications, AWS lambda, Azure function, Cloudflare
* Apollo Server can fetch data from any source you connect to (including a database, a REST API, a static object storage service, or even another GraphQL server). For other integrations: https://www.apollographql.com/docs/apollo-server/integrations/integration-index/

## Server
* Apollo Server can run in standalone `startStandaloneServer` or with other server like Express using `expressMiddleware`. But behind the hood, standalone use Express. Middle can be used to customize express configuration

## Resolvers
* Resolvers receive the following arguments:
    * parent: The return value of the resolver for this field's parent (i.e., the previous resolver in the resolver chain). For resolvers of top-level fields with no parent (such as fields of Query), this value is obtained from the rootValue function passed to Apollo Server's constructor.
    * args: An object that contains all GraphQL arguments provided for this field. For example, when executing query{ user(id: "4") }, the args object passed to the user resolver is { "id": "4" }.
    * contextValue: An object shared across all resolvers that are executing for a particular operation. Use this to share per-operation state, including authentication information, dataloader instances, and anything else to track across resolvers. See The contextValue argument for more information.
    * info: Contains information about the operation's execution state, including the field name, the path to the field from the root, and more. Its core fields are listed in the GraphQL.js source code. Apollo Server extends it with a cacheControl field.
* Resolvers can return the following responses:
    * Scalar / object: A resolver can return a single value or an object, as shown in Defining a resolver. This return value is passed down to any nested resolvers via the parent argument.
    * Array: Return an array if and only if your schema indicates that the resolver's associated field contains a list. After you return an array, Apollo Server executes nested resolvers for each item in the array.
    * null / undefined: Indicates that the value for the field could not be found. If your schema indicates that this resolver's field is nullable, then the operation result has a null value at the field's position. If this resolver's field is not nullable, Apollo Server sets the field's parent to null. If necessary, this process continues up the resolver chain until it reaches a field that is nullable. This ensures that a response never includes a null value for a non-nullable field. When this happens, the response's errors property will be populated with relevant errors concerning the nullability of that field.
    * Promise: Resolvers can be asynchronous and perform async actions, such as fetching from a database or back-end API. To support this, a resolver can return a promise that resolves to any other supported return type.

## Context and contextValue
* Allow to share context like authentication scope, sources of data
* Context is asynchronous and can be passed to `startStandaloneServer` or `expressMiddleware`
* Context initialization is called for every operation (request). It means that if a datasource is `new`ed in context, a new instance of datasource is created for every operation
* contextValue is accessible to plugins and resolvers.
* Resolvers can access the context with the third argument
```
const resolvers = {
  Query: {
    // All of our resolvers can access our shared contextValue!
    dogs: (_, __, contextValue) => {
      return contextValue.dataSources.animalApi.getDogs();
    },
    cats: (_, __, contextValue) => {
      return contextValue.dataSources.animalApi.getCats();
    },
  },
};

interface MyContext {
  // Context typing
  dataSources: {
    animalApi: AnimalAPI;
  };
}

const server = new ApolloServer<MyContext>({
  typeDefs,
  resolvers,
});

const { url } = await startStandaloneServer(server, {
  context: async () => {
    const animalApi = new AnimalAPI();
    return {
      dataSources: {
        animalApi,
      },
    };
  },
});
```
* Plugins can access the context
```
interface MyContext {
  token: string;
}

const server = new ApolloServer<MyContext>({
  typeDefs,
  resolvers: {
    Query: {
      hello: (root, args, { token }) => {
        return token;
      },
    },
  },
  plugins: [
    {
      async requestDidStart({ contextValue }) {
        // token is properly inferred as a string
        console.log(contextValue.token);
      },
    },
  ],
});

const { url } = await startStandaloneServer(server, {
  context: async ({ req, res }) => ({
    token: await getTokenForRequest(req),
  }),
});
```

## Errors
* ApolloServer add errors in its response as array to allow analysing each error that occured
* Built in errors: https://www.apollographql.com/docs/apollo-server/data/errors/#built-in-error-codes
* Stacktrace is not sent in errors if NODE_ENV is production or test. To force always appending stack trace or always omitting, use `includeStacktraceInErrorResponses`
* There is a `formatError` hook that can be applied to each error and passed to ApolloServer constructor. It is to format to client and not Apollo Studio

## Subscriptions
* ApolloServer does not offer built in support for subscriptions. It is possible to use librairies such as `graphql-ws`
* ApolloServer standalone does not support subscription. Have to use `expressMiddleware`
* Subscriptions can be handled by AsyncIterators `async function* () { }`
* Subscriptions can be handled by PubSub. But for production, it is recommanded to use subclasses of PubSub: https://www.apollographql.com/docs/apollo-server/data/subscriptions/#production-pubsub-libraries
* Events of pubsub can be filtered with `withFilter`
* Subscription resolvers have a context parameter to for instance check authorizations. Context is called once per subscription and not once per message
* ApolloServer can define `onConnect` and `onDisconnect` functions to modify behavior of connexions like for rejecting some clients

## Fetching data from sources
* ApolloServer can fetch data from any source like REST and databases
* Datasources can be configure at ApolloServer configuration to allow resolvers access it
```
interface ContextValue {
  dataSources: {
    dogsDB: DogsDataSource;
    catsApi: CatsAPI;
  };
  token: string;
}

const server = new ApolloServer<ContextValue>({
  typeDefs,
  resolvers,
});

const { url } = await startStandaloneServer(server, {
  context: async ({ req }) => {
     const { cache } = server;
    const token = req.headers.token;
    return {
      // We create new instances of our data sources with each request.
      // We can pass in our server's cache, contextValue, or any other
      // info our data sources require.
      dataSources: {
        dogsDB: new DogsDataSource({ cache, token }),
        catsApi: new CatsAPI({ cache }),
      },
      token,
    };
  },
});
```
* Some datasources available: 
    * https://github.com/apollographql/datasource-rest
    * https://github.com/nic-jennings/batched-sql-datasource
    * https://github.com/swantzter/apollo-datasource-firestore

### REST datasource
* https://github.com/apollographql/datasource-rest
* With datasource rest dependency, it is possible to use built in methods to fetch data or access `fetch` methods to have more complete objects
* Context initialization is called for every operation. It means that if a datasource is `new`ed in context, a new instance of datasource is created for every operation
* Librariy has two levels of cache configurable with `requestDeduplicationPolicyFor`: deduplication of GET and caching results
* As response are cached, that is why a new datasource is created for every request. Otherwise, there can be a mix of responses
* URL cached can be overriden
```
protected override requestDeduplicationPolicyFor(url: URL, request: RequestOptions) {
    const cacheKey = this.cacheKeyFor(url, request);
    return {
      policy: 'deduplicate-until-invalidated',
      deduplicationKey: `${request.method ?? 'GET'} ${cacheKey}`,
    };
  }

protected override requestDeduplicationPolicyFor(url: URL, request: RequestOptions) {
    const cacheKey = this.cacheKeyFor(url, request);
    return { policy: 'do-not-deduplicate' } as const;
  }
```
* `cache` object passed in datasources allows to specify which cache to use
* Every request can have `willSendRequest` to be used to add interceptor like to add headers
```
class PersonalizationAPI extends RESTDataSource {
  override baseURL = 'https://movies-api.example.com/';
  private token: string;

  constructor(options: { token: string; cache: KeyValueCache }) {
    super(options);
    this.token = options.token;
  }

  override willSendRequest(_path: string, request: AugmentedRequest) {
    request.headers['authorization'] = this.token;
  }
}
```

## Batching, caching, deduplication
* To perform these operation, use : https://github.com/graphql/dataloader
* It helps solving N+1 problems
* Dataloader is per request, have to create a new instance per request
```
import DataLoader from 'dataloader';

class ProductsDataSource {
  private dbConnection;

  constructor(dbConnection) {
    this.dbConnection = dbConnection;
  }

  private batchProducts = new DataLoader(async (ids) => {
    const productList = await this.dbConnection.fetchAllKeys(ids);
    // Dataloader expects you to return a list with the results ordered just like the list in the arguments were
    // Since the database might return the results in a different order the following code sorts the results accordingly
    const productIdToProductMap = productList.reduce((mapping, product) => {
      mapping[product.id] = product;
      return mapping;
    }, {});
    return ids.map((id) => productIdToProductMap[id]);
  });

  async getProductFor(id) {
    return this.batchProducts.load(id);
  }
}

// In your server file

// Set up our database, instantiate our connection,
// and return that database connection
const dbConnection = initializeDBConnection();

const { url } = await startStandaloneServer(server, {
  context: async () => {
    return {
      dataSources: {
        // Create a new instance of our data source for every request!
        // (We pass in the database connection because we don't need
        // a new connection for every request.)
         productsDb: new ProductsDataSource(dbConnection),
      },
    };
  },
});
```

## Notes
* Context passed to `new ApolloWerver` is used to type the context
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
* ApolloServer does not provide built in support for custom directive. Have to use tools like `@graphql-tools`
* If there is no resolver for a given field, ApolloServer will use a default one
* ApolloStudio est un outil qui permet d'avoir beaucoup d'insights comme les performances
