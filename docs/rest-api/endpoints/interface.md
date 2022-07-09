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
  			"name": "$IFACE_NAME",
  			"active": INT,
  			"created": "YYYY-MMM-DD hh:mm:ss",
  			"updated": "YYYY-MMM-DD hh:mm:ss",
  			"rxcounter": INT,
  			"txcounter": INT,
  			"rxtotal": INT,
  			"txtotal": INT
  		},
        ...
  	]
  }
  ```
