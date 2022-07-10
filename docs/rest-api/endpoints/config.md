## GET /api/config

get vnStat configuration

- Curl

  ```
  curl --location --request GET '$IP_ADDR:$PORT/api/config' \
  --header 'Authorization: Bearer 7VUXUiAQy1BunDkEe38q0qEOKWaKlNMi'
  ```

- Response body

  ```json
    {
      "status": "success",
      "data": {
        "TransparentBg": "0",
        "CTxD": "\"-\"",
        "SaveOnStatusChange": "1",
        "PidFile": "\"/var/run/vnstat/vnstat.pid\"",
        "DatabaseWriteAheadLogging": "0",
        "BootVariation": "15",
        "UpdateFileOwner": "1",
        ...

      }
    }
  ```

## PUT /api/config

edit vnStat configuration

- Curl

  ```
  curl --location --request PUT 'localhost:8080/api/config' \
    --header 'Authorization: Bearer $API_KEY' \
    --header 'Content-Type: application/json' \
    --data-raw '[
        {
            "prop": "$PROPERTY",       // Ex: "HourlyDays"
            "value" "$NEW_VALUE" // Ex: "-1"
        },
    ]'
  ```

- Response body

  ```json
  {
  	"status": "success",
  	"data": {
  		"HourlyDays": "-1"
  	}
  }
  ```
