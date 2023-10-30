# GraphQL Subscriptions

## What are subscriptions
* Subscription are long lasting read operations. 
* It is like Server side events. 
* New data are pushed from the server
* It uses WebSocket protocol
* Example of subscription type
```
type Subscription {
  postCreated: Post
}

Usage:
subscription PostFeed {
  postCreated {
    author
    comment
  }
}
```
* Each subscription operation can subscribe to only one field
* Example of resolver
```
const resolvers = {
  Subscription: {
    hello: {
      // Example using an async generator
      subscribe: async function* () {
        for await (const word of ['Hello', 'Bonjour', 'Ciao']) {
          yield { hello: word };
        }
      },
    },
    postCreated: {
      // More on pubsub below
      subscribe: () => pubsub.asyncIterator(['POST_CREATED']),
    },
  },
  // ...other resolvers...
};
```
* It is possible to use an internal bus using PubSub
```
Type:
type Subscription {
    post: Post!
}

For subscription:
post: {
    subscribe: (parent: any, args: any, ctx: any, info: any) => {
      return ctx.pubSub.subscribe('post');
    }
  }

For publishing events
ctx.pubsub.publish('post', {
        post: {
          mutation: 'CREATED',
          data: post
        }
      });

Usage:
subscription {
  post {
    body
  }
}

mutation {
  createPost(...)
}
```