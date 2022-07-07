# Getting started

## Installation

### From source

> #### Requirements

- rust (^1.62)
- cargo (^1.62)
- git

1. Clone repository

   ```terminal
   git clone https://github.com/Hulxv/vnstat-server.git vns
   ```

2. Build project

   ```terminal
   cd vns
   cargo build --release -p vns -p vnsd
   ```

3. Setup

   ```terminal
   cd target/release
   mv vns vnsd /bin/
   ```

---

## Usage

Before using vnStat-server, you should know that there are two programs, one is used as a daemon and another is used as a utility to do some operations like controlling the HTTP server ([see](./cli.md) for more).

### vnsd

It's a daemon that is used to run an HTTP server (RESTful API) that is used to control in vnStat and get its data by HTTP requests.

### vns

a Utility used by the end-user to control in HTTP server efficiently and easily by connecting with vnsd by [unix-socket](https://man7.org/linux/man-pages/man7/unix.7.html).

## More

- [RESTful API documentation](./rest-api/index.md)
- [Command line interface](./cli.md)
