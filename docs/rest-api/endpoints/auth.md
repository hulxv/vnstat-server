# POST /api/auth/login

- Request body
  ```json
  {
    "password": String // Your password
  }
  ```
- Response body

  ```json
  {
  	"status": "success",
  	"data": {
  		"uuid": String, // Connection UUID
  		"key": {
  			"value": String, // Key value
  			"expires_at": String // Key expire date
  		}
  	}
  }
  ```

## Related

- [Authentication](../authentication.md)
