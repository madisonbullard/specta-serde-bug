Expected:
```ts
export type Error = { code: "apiError", message: string } | { code: "unauthorized", message: string }
```

Received:
```ts
export type Error = { ApiError: string } | { Unauthorized: string }
```
