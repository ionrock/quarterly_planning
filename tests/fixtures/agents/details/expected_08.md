---
id: "test-008"
title: "GraphQL API Gateway"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Create a GraphQL API gateway that aggregates multiple backend REST services. Provides a unified query interface for clients, handles data fetching and composition. Built with Node.js and Apollo Server.

## Constraints

- Response time under 200ms for simple queries
- Must support both queries and mutations

## Implementation Notes

### Technology Stack
- **Runtime:** Node.js 20 LTS with TypeScript
- **GraphQL Server:** Apollo Server 4.x
- **HTTP Client:** axios with retry logic
- **Caching:** DataLoader for request-level batching
- **Monitoring:** Apollo Studio for tracing

### GraphQL Schema
```graphql
type Query {
  user(id: ID!): User
  users(limit: Int = 10, offset: Int = 0): UserConnection!
  product(id: ID!): Product
  products(categoryId: ID, limit: Int = 10): ProductConnection!
  order(id: ID!): Order
  orders(userId: ID!, status: OrderStatus): [Order!]!
}

type Mutation {
  createOrder(input: CreateOrderInput!): Order!
  updateOrderStatus(id: ID!, status: OrderStatus!): Order!
  updateUser(id: ID!, input: UpdateUserInput!): User!
}

type User {
  id: ID!
  email: String!
  name: String!
  orders: [Order!]!
  createdAt: DateTime!
}

type Product {
  id: ID!
  name: String!
  price: Money!
  category: Category!
  inventory: Int!
}

type Order {
  id: ID!
  user: User!
  items: [OrderItem!]!
  status: OrderStatus!
  total: Money!
  createdAt: DateTime!
}

type OrderItem {
  product: Product!
  quantity: Int!
  unitPrice: Money!
}

enum OrderStatus {
  PENDING
  CONFIRMED
  SHIPPED
  DELIVERED
  CANCELLED
}

scalar DateTime
scalar Money
```

### DataLoader Setup
```typescript
import DataLoader from 'dataloader';

interface DataLoaders {
  userLoader: DataLoader<string, User>;
  productLoader: DataLoader<string, Product>;
  ordersByUserLoader: DataLoader<string, Order[]>;
}

export function createLoaders(context: Context): DataLoaders {
  return {
    userLoader: new DataLoader(async (ids) => {
      const users = await userService.getByIds(ids as string[]);
      const userMap = new Map(users.map(u => [u.id, u]));
      return ids.map(id => userMap.get(id as string) ?? null);
    }),

    productLoader: new DataLoader(async (ids) => {
      const products = await productService.getByIds(ids as string[]);
      const productMap = new Map(products.map(p => [p.id, p]));
      return ids.map(id => productMap.get(id as string) ?? null);
    }),

    ordersByUserLoader: new DataLoader(async (userIds) => {
      const orders = await orderService.getByUserIds(userIds as string[]);
      const grouped = groupBy(orders, 'userId');
      return userIds.map(id => grouped[id as string] ?? []);
    }),
  };
}
```

### Resolver Implementation
```typescript
const resolvers: Resolvers = {
  Query: {
    user: async (_, { id }, { loaders }) => {
      return loaders.userLoader.load(id);
    },

    products: async (_, { categoryId, limit }, { services }) => {
      const products = await services.product.list({ categoryId, limit });
      return {
        edges: products.map(p => ({ node: p, cursor: p.id })),
        pageInfo: {
          hasNextPage: products.length === limit,
          endCursor: products[products.length - 1]?.id,
        },
      };
    },
  },

  User: {
    orders: async (user, _, { loaders }) => {
      return loaders.ordersByUserLoader.load(user.id);
    },
  },

  Order: {
    user: async (order, _, { loaders }) => {
      return loaders.userLoader.load(order.userId);
    },

    items: async (order, _, { loaders }) => {
      // Batch load all products for order items
      const productIds = order.items.map(i => i.productId);
      const products = await Promise.all(
        productIds.map(id => loaders.productLoader.load(id))
      );
      return order.items.map((item, i) => ({
        ...item,
        product: products[i],
      }));
    },
  },

  Mutation: {
    createOrder: async (_, { input }, { services, user }) => {
      if (!user) throw new AuthenticationError('Must be logged in');
      return services.order.create(user.id, input);
    },
  },
};
```

### Backend Service Client
```typescript
class UserService {
  private client: AxiosInstance;

  constructor(baseURL: string) {
    this.client = axios.create({
      baseURL,
      timeout: 5000,
      headers: { 'Content-Type': 'application/json' },
    });

    // Retry on failure
    axiosRetry(this.client, {
      retries: 2,
      retryDelay: axiosRetry.exponentialDelay,
      retryCondition: (error) =>
        axiosRetry.isNetworkOrIdempotentRequestError(error) ||
        error.response?.status === 503,
    });
  }

  async getByIds(ids: string[]): Promise<User[]> {
    const response = await this.client.get('/users', {
      params: { ids: ids.join(',') },
    });
    return response.data;
  }
}
```

### Query Complexity Limiting
```typescript
import { createComplexityLimitRule } from 'graphql-validation-complexity';

const complexityLimitRule = createComplexityLimitRule(1000, {
  scalarCost: 1,
  objectCost: 2,
  listFactor: 10,
  onCost: (cost) => console.log('Query complexity:', cost),
});

const server = new ApolloServer({
  typeDefs,
  resolvers,
  validationRules: [complexityLimitRule],
});
```

### Error Handling
```typescript
const formatError = (error: GraphQLError): GraphQLFormattedError => {
  // Log full error internally
  logger.error('GraphQL error', {
    message: error.message,
    path: error.path,
    extensions: error.extensions,
  });

  // Return sanitized error to client
  if (error.extensions?.code === 'INTERNAL_SERVER_ERROR') {
    return {
      message: 'An unexpected error occurred',
      extensions: { code: 'INTERNAL_SERVER_ERROR' },
    };
  }

  return error;
};
```

### Context Setup
```typescript
interface Context {
  user: User | null;
  loaders: DataLoaders;
  services: Services;
}

const server = new ApolloServer({
  typeDefs,
  resolvers,
  context: async ({ req }): Promise<Context> => {
    const user = await authenticateRequest(req);
    return {
      user,
      loaders: createLoaders(),
      services: createServices(),
    };
  },
});
```

## Review Notes

(none yet)

## Tickets

### Ticket 1: Schema Design

**Summary:** Define GraphQL schema based on backend data models.

**Definition of Done:** Schema compiles and represents all required types.

### Ticket 2: Resolver Implementation

**Summary:** Create resolvers that fetch data from backend services.

**Definition of Done:** All queries return correct data from backends.

### Ticket 3: Performance Optimization

**Summary:** Add DataLoader for batching and caching.

**Definition of Done:** N+1 queries are eliminated.
