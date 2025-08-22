## Bash Commands
- `npm i`: Install the project and the dependencies
- `npm test`: Run the entire testing suite
- `npm test -- --testNamePattern="client.exampleResource.exampleMethod"` 
- `npm add (dependency_name)`: Add a new dependency

## Definitions
- "the client": The main exported module e.g. `import MyClient from "my_sdk"`
- "client function": An SDK function that the client can access
- "root client": `client.ts` at the root of the main module

## Code style
- All imports should be placed at the top of a file
- Add a type to every function argument
- Add a return type to every function
- Add a docstring to every new function

## Types Guide
- boolean, number, string for primitives
- string literals with "|" for string enums
- [] for arrays
- "|" for unions

### Argument Types Guide
- `import { type UploadFile } from "<sdk_name>/core";` for binary data
- For object inputs, create a file called `argument-types.ts` at the same level as the client and export the created type from `index.ts`

### Response Types Guide
- `import { BinaryResponse } from "<sdk_name>/core";` for binary data
- For object responses, create a new file and type in the `src/types` folder.
- Export any newly created types from `src/types/index.ts`

## Client functions
- Functions should take at max two inputs. The first is a type for input data, and the second is `opts?: RequestOptions` (`import { RequestOptions} from  "<sdk_name>/core";`)

## Tests
- Create jest test for any new functionality in `test/`
- Tests run against a stateless mock server, so never make any stateful assertions in the tests
- Create contract tests that test that the request is successful when all parameters are given and when only required parameters are given

## Documentation
- When creating a new resource, include a README file in the resource folder
- See the [example readme](#documentation) for the style

## IMPORTANT RULES
- Do not remove or change the versions of `form-data` `form-url-encoded` `js-base64` `jsonpointer` `node-fetch` `qs` or `zod`
- Any imports that are node or browser dependent must be imported using the `require` keyword inline according to the runtime from (`import { RUNTIME } from "<sdk_name>/core";`)
- Prefer creating a new resource when chaining together multiple API calls
- Do not change any code in `"src/core"`;
- Do not change `src/environment.ts`
- Never use this._client.makeRequest directly, always use existing functionalities via resource clients

## Code Examples

### Type
```ts
import * as z from "zod";

import { zodTransform } from "<sdk_name>/portal-client/core";

/**
 * Example Type
 */
export type Example = {
  /**
   * the unique identifier of this example
   */
  id: string;
};
```

### Function Signature
  /**
   * Functionality Description
   */
  method(
    request: requests.MethodRequest,
    opts?: RequestOptions,
  ): Promise<null> {
  }

### New Resource
Files
- index: `src/resources/example-resource/index.ts`
- client: `src/resources/example-resource/resource-client.ts`
- request types: `src/resources/example-resource/request-types.ts`
- docs: `src/resources/example-resource/README.md`

Example `index.ts`
```ts
export {
  ExampleMethodRequest,
} from "./request-types";
export { ExampleClient } from "./resource-client";
```

Example `request-types.ts`
```ts
export type ExampleMethodRequest = {
  id: string;
  val?: string;
}
```

Example `resource-client.ts`
```ts
import type { types } from "<sdk_name>";
import {
  CoreResourceClient,
  type RequestOptions,
} from "<sdk_name>/core";
import type * as requests from "<sdk_name>/resources/example-resource/request-types";
import { ResourceClientA } from "<sdk_name>/resources/a";
import { ResourceClientB } from "<sdk_name>/resources/b";


export class ExampleClient extends CoreResourceClient {
  /**
   * Example Description
   */
  async exampleMethod(
    request: requests.ExampleMethodRequest = {},
    opts?: RequestOptions,
  ): Promise<types.ExampleMethodOutput> {
    const resourceA = new ResourceClientA(this._client, this._opts);
    const resourceB = new ResourceClientB(this._client, this._opts);

    const resA = await resourceA.get({ id: request.id });
    const resB = await resourceB.create({ a: resA.id, val: request.val });

    return { aId: resA.id, bId: resB.id };
  }
}

```

### Tests
`test/example-resource.test.ts`
```ts
import Client, { Environment } from "<sdk_name>";

describe("tests client.exampleResource.exampleMethod", () => {
  test.concurrent(
    "Desciption actions | testId: success_all_params | Desciption of test",
    async () => {
      const client = new Client({
        apiKey: "API_KEY",
        environment: Environment.MockServer,
      });
      const response = await client.exampleResource.exampleMethod( 
        {
          id: "abc",
          val: "data",
        }
      );
      expect(response).toBeDefined();
    }
  )
  test.concurrent(
    "Desciption actions | testId: success_required_only | Desciption of test",
    async () => {
      const client = new Client({
        apiKey: "API_KEY",
        environment: Environment.MockServer,
      });
      const response = await client.exampleResource.exampleMethod({ id: "abc" });
      expect(response).toBeDefined();
    }
  )
});
```

### Documentation
`src/resources/example-resource/README.md`

````markdown
# example-resource

## Module Functions
### Example Method <a name="exampleMethod"></a>

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|--------|
| `id` | ✓ | the unique idenifier | `"3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a"` |
| `val` | ✗ | the example to send | `data` |


#### Example Snippet

```typescript
import Client from "<sdk_name>";

const client = new Client({
  apiKey: process.env["API_KEY"]!!,
});
const res = await client.exampleResource.exampleMethod({
  id: "3e4666bf-d5e5-4aa7-b8ce-cefe41c7568a"
});

```

#### Response

##### Type
[ExampleMethodOutput](/src/types/example-method-output.ts)

##### Example
`{ aId: "abc", bId: "bcd" }`

````