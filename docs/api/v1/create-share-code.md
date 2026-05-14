# POST /api/v1/create-share-code

### request
```jsonc
{ "metoffice-api-key": string } // met office api key
```

### response (success)
This is expected, HTTP response code 201
```jsonc
{ "kind": "created", "share-code" : string }
```

### response (failure)
This should never happen, but HTTP response code 500
```jsonc
{ "kind": "error", "reason": string }
```
