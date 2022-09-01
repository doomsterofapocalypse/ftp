USAGE:
    ftp.exe [OPTIONS] --mode <MODE>

OPTIONS:
    -c, --cert <CERT>            path to the cert.pem file
    -f, --filepath <FILEPATH>    filepath of the file to transfer
    -h, --https                  use https protocol - default port of 443 is used
        --help                   Print help information
    -i, --ip <IP>                IP address to connect (to be used with client mode)
    -k, --key <KEY>              path to key.pem file
    -m, --mode <MODE>            mode to start the executable in either server or client
    -p, --port <PORT>            port number to use (mandatory to start server)
    -t, --tcp                    use TCP protocol - DEFAULT if no protocol is specifiec
    -u, --udp                    Use UDP protocol
    -V, --version                Print version information
