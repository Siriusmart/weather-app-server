# POST /api/v1/revoke-share-code

### request
```jsonc
{ "metoffice-api-key": string } // met office api key
```

### response (success)
This is expected, HTTP response code 200
```jsonc
{ "kind": "revoked", "share-code" : string }
```

### response (failure)
No share code with key, HTTP response code 404
```jsonc
{ "kind": "error", "reason": "not found" }
```

### response (failure)
This should never happen, but HTTP response code 500
```jsonc
{ "kind": "error", "reason": string }
```
