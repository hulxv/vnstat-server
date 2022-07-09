# GET /api/info

Get vnStat information

- Curl

  ```
  curl --location --request GET '$IP_ADDR:$PORT/api/info' \
  --header 'Authorization: Bearer $API_KEY'
  ```

- Response

  ```json
  {
  	"status": "success",
  	"data": [
  		{
  			"id": 1,
  			"name": "btime",
  			"value": "1657317065"
  		},
  		{
  			"id": 2,
  			"name": "dbversion",
  			"value": "1"
  		},
  		{
  			"id": 3,
  			"name": "vnstatversion",
  			"value": "2.9"
  		}
  	]
  }
  ```
