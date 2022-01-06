# ryna

`ryna` is a Nginx log analyzer implemented by Rust.

This log analyzer aims to provide a quick overview for the statistical data generated from Nginx's access log.

## Installation

### From Source

If you are a Rustacean, the installation will be quite easy.

This repository use [Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) as the build system. If you want to get binary from source, simply use `cargo build`.

## Quick Example

This example uses the example access log and config in `ryna/res`.

An quick example will start like the following:

```shell
./ryna -c res/config.yaml -a res/access.log
```

Which returns you a simple overview for access.log in JSON:

```json
[{"title":"$my_trans_id","count":100},{"title":"$uri","count":100,"map":{"/cargo/test.html":22,"/ComplexOper.action":1,"/primitives/tuples":36,"/std_misc/arg.html":41}},{"title":"$time_local","count":100,"map":{"20/Apr/2021:23:59":28,"21/Apr/2021:00:00":53,"21/Apr/2021:00:01":19}},{"title":"$args","count":100},{"title":"$server_protocol","count":100,"map":{"HTTP/1.1":100}},{"title":"$my_http_header_field","count":100,"map":{"GetUID":34,"GetMail":18,"GetByCond":21,"GetAbsList":1,"GetAbstract":4,"CheckAPI":20,"GetDomainId":2}},{"title":"$server_name","count":100,"map":{"my-service.com":100}},{"title":"$remote_port","count":100},{"title":"$body_bytes_sent","total":9318505,"count":100,"mean":93185.05,"variance":66570583829344.766,"max":8187054,"min":14},{"title":"$http_referer","count":100},{"title":"$remote_addr","count":100,"map":{"10.64.10.221":42,"10.64.101.36":12,"10.64.12.118":46}},{"title":"$request_time","count":100},{"title":"$status","count":100,"map":{"400":4,"200":96}},{"title":"$upstream_response_time","total":2.736999999999997,"count":100,"mean":0.027369999999999995,"variance":0.3820853099999999,"max":0.428,"min":0.004},{"title":"$request_method","count":100,"map":{"GET":100}},{"title":"$upstream_addr","count":100,"map":{"172.27.18.51:8080":21,"172.27.18.18:8080":20,"172.27.8.20:8080":19,"172.27.19.18:8080":20,"172.27.18.50:8080":20}}]
```

Simple, but enough for most of time if you want to locate what problems may have occurred.

For more information about command line arguments, use:

```shell
./ryna -h
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
$remote_addr $remote_port [$time_local] $"request" $status $body_bytes_sent "$http_referer" "$http_user_agent"
```

And the type format will be like:

```txt
str,str,thour,str,str,noop,str,str,isize,noop,noop,str,str,f64,noop,str
```

Which means that you need to `escape` the `log_format` string written in your Nginx config section to the analyzer's log format file. Then, apply types to the fileds correspondly and seperate them by a comma. If you want to ignore some specific fileds, apply `noop` type to them.

Check the full example config in `ryna/res/config.yaml`.

### Use Correct Type

The analyzer provides 6 types for data in the access log:

- `noop`, which means skipping the log data.
- `str`, which means the data is in type string.
- `isize`, which means the data is in integer with sign.
- `f64`, which means the data is in 64-bit float.
- `hour`, which means the data is Nginx time_local in hours.
- `min`, which means the data is Nginx time_loacl in minutes.

## Others

Feel free to launch any issue or pull request.

Hope this little tool save you some time.
