# Command Line Interface

## vns (vnStat Server CLI utility)

a Utility used by the end-user to control in HTTP server efficiently and easily by connecting with vnsd by [unix-socket](https://man7.org/linux/man-pages/man7/unix.7.html).

- help

  ```
  vns 0.1.0
    Mohamed Emad (hulxxv@gmail.com)

    a Utility used by the end-user to control in vnStat HTTP server efficiently and easily by
    commmunicate with vnsd by unix-socket.

    USAGE:
        vns [SUBCOMMAND]

    OPTIONS:
        -h, --help       Print help information
        -V, --version    Print version information

    SUBCOMMANDS:
        help      Print this message or the help of the given subcommand(s)
        server    To controlling in your vns HTTP server
  ```

- subcommands

  - help

    Print this message or the help of the given subcommand(s)

    - Usage:

      ```
      $ vns help [SUBCOMMAND]...
      ```

    - Args:
      ```
      <SUBCOMMAND>...    The subcommand whose help message to display
      ```

  - server

    To controlling in your vns HTTP server.

    - Usage:
      ```
      $ vns server <COMMAND>
      ```
    - Commands:

      ```
      help
          Print this message or the help of the given subcommand(s)
      list <connections|block>
          get list of blocks or connections in vns HTTP server.
      pause
          Pause accepting incoming connections. May drop socket pending connection. All
          open connections remain active.

      resume
          Resume accepting incoming connections.

      shutdown
          Shutdown server. You will need to restart vns daemon to running the server again.

      status
          Get vns HTTP server status.

      block <LIST_OF_IP_ADDRESSES>
          Block specific ip address to disallow using HTTP server.

      un-block <LIST_OF_IP_ADDRESSES>
          un-Block specific ip address that was blocked and allow using HTTP server again.
      ```

## vnsd (vnStat Server Daemon)

It's a daemon that is used to run an HTTP server (RESTful API) that is used to control in vnStat and get its data by HTTP requests.

- help

  ```
  vnsd 0.1.0
  Mohamed Emad (hulxxv@gmail.com)
  vnStat Server daemon

  USAGE:
      vnsd [OPTIONS]

  OPTIONS:
      -h, --help           Print help information
          --ip <IP>        select pid file
          --port <PORT>    set daemon process user
      -V, --version        Print version information
  ```
