## Bash Commands
- `poetry install`: Install the project and the dependencies
- `poetry run pytest`: Run the testing suite
- `poetry run mypy <module_name>/`: Run the type checker on the main module
- `poetry run pytest tests/test_specific_file.py -v`: Run specific test file
- `poetry add (dependency_name)`: Add a new dependency

## Definitions
- "the client": The main exported module e.g. `from my_sdk import MyClient`
- "client function": An SDK function that the client can access
- "base client" The sync and async client in `client.py` at the root of the main module

## Code style
- All imports should be placed at the top of a file
- Add a type hint to every function argument
- Add a type hint for every function response
- Create a sync and an async version of all client functions

## Types Guide
- bool, int, float, str for primitives
- typing_extensions.Literal["option1", "option2"] for string enums
- typing.List[T] for arrays
- typing.Union[Type1, Type2, ...] for unions
- "MyClass" (quoted) for self-referencing objects

### Argument Types Guide
- Use `httpx._types.FileTypes` for binary data
- For object inputs, create a typed dict class in `types/params` (see [typed-dicts](#typeddict)) and import it and use it like this: `param_name: params.MyExampleParam`
- Optional/Nullable: Wrap in typing.Optional[T]
 
### Response Types Guide
- `BinaryResponse` (imported from core) for binary data
- For object inputs, create a pydantic model class in `types/models` (see [pydantic model](#pydantic-model)) and use it like this: `param_name: models.MyExampleModel`

## Client function signatures
- See [function signature example](#function-signature)
- Start with "self, *" to prevent positional args
- The last arg should always be `request_options: typing.Optional[RequestOptions] = None,`
- Complex input types go in the `types/params` folder. These are always typed dicts
- Complex response types go in the `types/models/` folder. These are always pydantic models

## Client functions
- For functions that combine multiple API calls, import and instantiate the specific client classes rather than trying to access them through the base client
- Example: `from module.resources.api.spec.client import SpecClient; spec_client = SpecClient(base_client=self._base_client)`
- Include appropriate `auth_names` list
- Use `cast_to=type(None)` for methods returning None

## Tests
- Create tests for any new functionality in `tests/`
- Tests run against a stateless mock server, so never make any stateful assertions in the tests
- Create contract tests that tests that the request is successful when all parameters are given and when only required parameters are given
- Test behavior when all parameters test - Call method with all possible parameters
- Test behavior when all only required parameters test - Call method with only required parameters
- Always create Async versions  using @pytest.mark.asyncio


## Documentation
- When creating a new resource, include a README file in the resource folder
- See the [example readme](#documentation) for the style
- Add an internal link in the README.md file at the root of the repository

## Workflow
- Always typecheck when you're done making a series of code changes
- Prefer running single tests, and not the whole test suite
- Reference the README.md in the root of the repo to understand the module structure of the repo when gathering context. Follow the internal README links to view parameter tables, example snippets, and response examples.
- When adding a new resource: create a new folder within `resources/` create `client.py`, create `__init__.py` create `README.md` add the resource to the sync and async base clients.
- When creating new typed dicts or models, remember to add them to the respective `__init__.py` files in both the import section and `__all__` list
- Always run tests after creating new functionality to ensure everything works correctly

## IMPORTANT RULES
- Do not remove or change versions of pydantic, httpx in pyproject.toml
- Instantiate the specific client classes needed rather than trying to access them. Never access the base_client directly
- Prefer creating a new resource  when chaining together multiple API calls
- Remove unused imports to avoid linting errors
- When creating new models and params, always create new files (1 type per file)
- Always verify the correct Environment enum values before using in tests, it will usually be `Environment.MOCK_SERVER`
- When adding to an existing resource's README.md, add new content <!-- CUSTOM DOCS START --> and <!-- CUSTOM DOCS END --> comment so that the code generator can retain the content in future versions.

## Code Examples

### TypedDict
```py
import pydantic
import typing_extensions


class Example(typing_extensions.TypedDict):
    """
    Example type description
    """

    name: typing_extensions.Required[str]
    """
    A great description of the name parameter on this example typed dict
    """


class _SerializerExample(pydantic.BaseModel):
    """
    Serializer for Example handling case conversions
    and file omissions as dictated by the API
    """

    model_config = pydantic.ConfigDict(
        populate_by_name=True,
    )

    name: str = pydantic.Field(
        alias="name",
    )
```

### Pydantic Model
```py
import pydantic


class ModelExample(pydantic.BaseModel):
    """
    A description of the purpose of this example model
    """

    model_config = pydantic.ConfigDict(
        arbitrary_types_allowed=True,
        populate_by_name=True,
    )

    name: str = pydantic.Field(
        alias="name",
    )
    """
    A great description of the name field on this example model
    """
```

### Function signature
```py
   def create(
        self,
        *,
        asset: params.Asset,
        name: typing.Union[
            typing.Optional[str], type_utils.NotGiven
        ] = type_utils.NOT_GIVEN,
        request_options: typing.Optional[RequestOptions] = None,
    ) -> models.CreatedAssets:
```

### New resource
Example file location: `resources/your_resource/__init__.py`
```python
from .client import AsyncYourResourceClient, YourResourceClient

__all__ = ["AsyncYourResourceClient", "YourResourceClient"]
```

Example file location: `resources/your_resource/client.py`
```py
import typing
from <module_name>.core import (
    AsyncBaseClient,
    RequestOptions, 
    SyncBaseClient,
    type_utils,
)
from <module_name>.types import models

from <module_name>.resources.custom import CustomClient

class ResourceClient:
    def __init__(self, *, base_client: SyncBaseClient) -> None:
        self._base_client = base_client
    
    def your_new_method(
        self,
        *,
        required_param: str,
        optional_param: typing.Optional[str] = None,
        request_options: typing.Optional[RequestOptions] = None
    ) -> models.NewMethodResponse:
        """
        Brief description
        
        Longer description explaining the functionality.
        
        HTTP_METHOD /api/endpoint/path
        
        Args:
            required_param: Description of required parameter
            optional_param: Description of optional parameter  
            request_options: Additional options to customize the HTTP request
            
        Returns:
            Success response description
            
        Raises:
            ApiError: A custom exception class that provides additional context
                for API errors, including the HTTP status code and response body.
        """
        your_client = CustomClient(base_client=self._base_client)
        result = your_client.call_api(
            required_param=param1,
            optional_param=optional_param,
            request_options=request_options
        )
        return models.NewMethodResponse(data=result)

class AsyncYourResourceClient:
    def __init__(self, *, base_client: AsyncBaseClient) -> None:
        self._base_client = base_client
        
    async def your_new_method(
        self,
        *,
        required_param: str,
        optional_param: typing.Optional[str] = None,
        request_options: typing.Optional[RequestOptions] = None
    ) -> models.NewMethodResponse:
        """
        [Same docstring as sync version, but explain that it is async]
        """
        your_client = AsyncCustomClient(base_client=self._base_client)
        result = await your_client.call_api(
            required_param=param1,
            optional_param=optional_param,
            request_options=request_options
        )
        return models.NewMethodResponse(data=result)
```


### Tests
```python
import io
import pydantic
import pytest
import typing

from <module_name> import AsyncResourceClient, ResourceClient
from <module_name>.environment import Environment
from <module_name>.types import models


def test_create_201_success_all_params():
    """Tests a POST request to the /project/{name} endpoint.

    Operation: create
    Test Case ID: success_all_params
    Expected Status: 201
    Mode: Synchronous execution

    Empty response expected

    Validates:
    - Authentication requirements are satisfied
    - All required input parameters are properly handled
    - Response status code is correct
    - Response data matches expected schema

    This test uses example data to verify the endpoint behavior.
    """
    # tests calling sync method with example data
    client = ResourceClient(api_key="API_KEY", environment=Environment.MOCK_SERVER)
    response = client.resource.create(project="my-project", file=io.BytesIO(b"123"))
    try:
        pydantic.TypeAdapter(models.Project).validate_python(response)
        is_valid_response_schema = True
    except pydantic.ValidationError:
        is_valid_response_schema = False
    assert is_valid_response_schema, "failed response type check"


@pytest.mark.asyncio
async def test_await_create_201_success_all_params():
    """Tests a POST request to the /project/{name} endpoint.

    Operation: create
    Test Case ID: success_all_params
    Expected Status: 201
    Mode: Asynchronous execution

    Empty response expected

    Validates:
    - Authentication requirements are satisfied
    - All required input parameters are properly handled
    - Response status code is correct
    - Response data matches expected schema

    This test uses example data to verify the endpoint behavior.
    """
    # tests calling async method with example data
    client = ResourceClient(api_key="API_KEY", environment=Environment.MOCK_SERVER)
    response = await client.resource.create(project="my-project", file=io.BytesIO(b"123"))
    try:
        pydantic.TypeAdapter(models.Project).validate_python(response)
        is_valid_response_schema = True
    except pydantic.ValidationError:
        is_valid_response_schema = False
    assert is_valid_response_schema, "failed response type check"

```


### Documentation
```markdown
### Method Name <a name="method-name"></a>

Brief description of what the method does.

**API Endpoint**: `HTTP_METHOD /api/endpoint/path` (or all endpoints it will hit if it's a workflow)

#### Parameters

| Parameter | Required | Description | Example |
|-----------|:--------:|-------------|---------|
| `required_param` | ✓ | Description | `"example_value"` |
| `optional_param` |   | Description | `"optional_value"` |

#### Synchronous Client

```python
from <module_name> import <ClientName>
from os import getenv

client = <ClientName>(api_key=getenv("API_KEY"))
res = client.your_resource.method_name(required_param="value")
```

#### Asynchronous Client

```python  
from <module_name> import <AsyncClientName>
from os import getenv

client = <AsyncClientName>(api_key=getenv("API_KEY"))
res = await client.your_resource.method_name(required_param="value")
```

#### Response

##### Type
[YourResponseModel](/path/to/types/your_response_model.py)

##### Example
`{"field": "value", "another_field": 123}`
```