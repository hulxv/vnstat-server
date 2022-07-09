# Authentication

This part is the most important part. Because, It's protect your privacy and secure.

1. Set your password

   In your configuration file, There's an option called 'password' in 'auth' section.

   ```toml
   [auth]
   password = <String> # default: "password"
   ```

2. Get API Key

   By send HTTP request with POST method to '/api/auth/login/' and with password in request body, You will get your API key that you will use to use that RESTful API

3. Use API Key

   In authorization, We use [Bearer authentication](https://www.devopsschool.com/blog/what-is-bearer-token-and-how-it-works/). So you need to send your API key in Authrization header like that:

   ```
    Authrization: Bearer $API_KEY
   ```

## Connection

It's a concept similar to the user in social media platforms, but instead of relying on username or email and things like that, it's based on the IP address and user agent. So for each IP address or different user agent, will create a new connection for it.

## Key

For every connection, There is a key. And that key have an expire duration, You can controlling in the duration by 'key_expire_duration' in 'auth section.

```toml
[auth]
key_expire_duration = INT # By day
```
