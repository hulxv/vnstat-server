# GET /api/traffic/\<interval\>

- Curl

  ```
  curl --location --request GET 'localhost:8080/api/traffic/month' \
  --header 'Authorization: Bearer $API_KEY'
  ```

- Response

  ```json
  {
  	"status": "success",
  	"data": [
  		{
  			"id": INT,
  			"interface": INT,
  			"date": "YYYY-MM-DDD",
  			"rx": INT,
  			"tx": INT
  		},
        ...
  	]
  }
  ```
