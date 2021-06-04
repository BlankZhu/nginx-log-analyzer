# Nginx-Log-Analyzer

This repository contains a log analyzer implemented by Rust.

This log analyzer aims to provide a quick overview for the statistical data generated from Nginx's access log.

## Installation

### From Source

If you are a Rustacean, the installation will be quite easy.

This repository use [Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) as the build system. If you want to get binary from source, simply use `cargo build`.

### Pre-Compiled

For those who don't get a Rust develop environment, the [release page](https://github.com/BlankZhu/nginx-log-analyzer/releases) might helps (but not now).

## Quick Example

This example uses the example access log and format file in `nginx-log-analyzer/res`.

An quick example will start like the following:

```shell
./nginx-log-analyzer -l res/log.fmt -t res/typ.fmt -a res/access.log
```

Which returns you a simple overview for access.log:

```shell
remote_addr
count:  100
detail:
- 10.64.12.118  46.0000%
- 10.64.101.36  12.0000%
- 10.64.10.221  42.0000%
==========
remote_port
count:  100
detail:
- 39145 42.0000%
- 46270 46.0000%
- 48888 12.0000%
==========
time_local(Noop)
no avaliable information
==========
request_method
count:  100
detail:
- GET   100.0000%
==========
uri
count:  100
detail:
- /ComplexOper.action   1.0000%
- /cargo/test.html      22.0000%
- /primitives/tuples    36.0000%
- /std_misc/arg.html    41.0000%
==========
args(Noop)
no avaliable information
==========
server_protocol
count:  100
detail:
- HTTP/1.1      100.0000%
==========
status
count:  100
detail:
- 400   4.0000%
- 200   96.0000%
==========
body_bytes_sent
count:  100
max:    8187054
min:    14
total:  9318505
average:        93185.05
variance:       665705838293.4482
STD:    815907.9839623143
==========
http_referer(Noop)
no avaliable information
==========
my_trans_id(Noop)
no avaliable information
==========
my_http_header_field
count:  100
detail:
- "GetAbsList"  1.0000%
- "CheckAPI"    20.0000%
- "GetUID"      34.0000%
- "GetMail"     18.0000%
- "GetDomainId" 2.0000%
- "GetAbstract" 4.0000%
- "GetByCond"   21.0000%
==========
upstream_addr
count:  100
detail:
- 172.27.18.51:8080     21.0000%
- 172.27.19.18:8080     20.0000%
- 172.27.18.18:8080     20.0000%
- 172.27.18.50:8080     20.0000%
- 172.27.8.20:8080      19.0000%
==========
upstream_response_time
count:  100
max:    0.428
min:    0.004
total:  2.736999999999997
average:        0.02736999999999997
variance:       0.0038208531000000013
STD:    0.06181304959310777
==========
request_time
count:  100
max:    0.428
min:    0.004
total:  2.741999999999996
average:        0.02741999999999996
variance:       0.0038204035999999998
STD:    0.06180941352253716
==========
server_name
count:  100
detail:
- my-service.com        100.0000%
```

Simple, but enough for most of time if you want to locate what problems may  have occurred.

If you need more information about command line arguments, use:

```shell
./nginx-log-analyzer -h
```

## Apply Log Format

To adapt your own Nginx log format, there are only a few syntax rules for the log format file you need to know.

### Organize the Log Format File

The log format file is organized according to the Nginx's log format.

Take the following Nginx's log format as an example:

```nginx
# nginx.conf
http {
  ...
  log_format combined '$remote_addr $remote_user [$time_local] '
                      '"$request" $status $body_bytes_sent '
                      '"$http_referer" "$http_user_agent"';
  ...
}
```

The log format file used by the analyzer is organized sequentially in one line like below:

```txt
$remote_addr	$remote_port	[$time_local]	$request_method	$uri	$args	$server_protocol	$status	$body_bytes_sent	"$http_referer"	"$my_trans_id"	"$my_http_header_field"	$upstream_addr	$upstream_response_time	$request_time	$server_name
```

And the type format will be like:

```txt
str,str,noop,str,str,noop,str,str,isize,noop,noop,str,str,f64,noop,str
```

Which means that you need to `escape` the `log_format` string written in your Nginx config section to the analyzer's log format file. Then, apply types to the fileds correspondly and seperate them by a comma. If you want to ignore some specific fileds, apply `noop` type to them.


### Use Correct Type

The analyzer provides 4 types for data in the access log:

- `noop`, which means skipping the log data.
- `str`, which means the data is in type string.
- `isize`, which means the data is in integer with sign.
- `f64`, which means the data is in 64-bit float.

## Others

Feel free to launch any issue or pull request.

Hopt this little tool save you some time.