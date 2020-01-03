# compress_history
![crates.io](https://img.shields.io/crates/v/compress_history.svg)
## Example
```terminal
$ cat history 
echo "hallo"
echo "hello"
ping 192.168.0.1
ping 192.168.0.2
ping 192.168.0.3
ping 192.168.0.4
ls -l
ls -a
$ compress_history -f history 
echo "hello"
ping 192.168.0.4
ls -l
ls -a
```

## Usage
```terminal
$ compress_history --help
Toru3 <ryuutet@gmail.com>
delete unnecessary history

USAGE:
    compress_history [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <FILE>    Target history file [default: /home/user/.bash_history]
```

## LICENSE
MIT OR Apache-2.0
