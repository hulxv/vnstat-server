# RESTful API Documentation

Docoumentation manual to know how to use vnStat Server RESTful API.

## Response

- Success operation

  ```json
  {
  	"status": "success",
  	"data": <DATA>
  }
  ```

- Failed operation

  ```json
  {
      "status": "failed",
      "data": {
          "code": <ERROR_CODE>,
          "details": <ERROR_DETAILS>
      }
  }
  ```

## Structure

```text
http://<IP_ADDR>:<PORT>/
└── api
    ├── auth
    │   └── login       (POST)  ~> Authentication
    ├── traffic
    │   ├── fiveminutes (GET)   ~> Get traffic data per 5min
    │   ├── hour        (GET)   ~> Get traffic data per hour
    │   ├── day         (GET)   ~> Get traffic data per day
    │   ├── month       (GET)   ~> Get traffic data per month
    │   ├── year        (GET)   ~> Get traffic data per year
    │   └── top         (GET)   ~> Get top traffic usage data
    ├── info            (GET)   ~> Get vnStat Information
    ├── interface       (GET)   ~> Get vnStat interfaces data
    ├── configs       (GET|PUT) ~> Get/Edit vnStat configuration
    └── daemon          (GET)   ~> Get vnStatD status
        ├── stop        (POST)  ~> Stop vnStatD
        └── restart     (POST)  ~> Restart vnStatD
```

## Authentication

For security, this process ensures that unwanted people do not access your data.
Read [more](./authentication.md).

## Endpoints

- [Auth](./endpoints/auth.md)
- [Traffic](./endpoints/traffic.md)
- [Info](./endpoints/info.md)
- [Interface](./endpoints/interface.md)
- [Config](./endpoints/config.md)
- [Daemon](./endpoints/daemon.md)
