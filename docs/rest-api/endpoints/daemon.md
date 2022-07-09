## GET /api/daemon

get vnStatD status

- Curl

  ```
  curl --location --request GET '$IP_ADDR:$PORT/api/daemon' \
  --header 'Authorization: Bearer $API_KEY'
  ```

- Response body

  ```json
  {
  	"status": "success",
  	"data": {
  		"is_active": BOOL
  	}
  }
  ```

## POST /api/daemon/stop

edit vnStat configuration

- Curl

  ```
  curl --location --request POST 'localhost:8080/api/daemon/stop' \
    --header 'Authorization: Bearer $API_KEY'
  ```

- Response body

  ```json
  {
  	"status": "success",
  	"data": {
  		"details": "$MESSAGE"
  	}
  }
  ```

## POST /api/daemon/restart

edit vnStat configuration

- Curl

  ```
  curl --location --request POST 'localhost:8080/api/daemon/restart' \
    --header 'Authorization: Bearer $API_KEY'
  ```

- Response body

  ```json
  {
  	"status": "success",
  	"data": {
  		"details": "$MESSAGE"
  	}
  }
  ```
