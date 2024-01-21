# traffic-processor-pods-checker

A simple program that summarizes the status of traffic-processor pods for a list of tenants

Usage: `target/release/check-traffic-processor-pods "foo,bar"`

For instance, given the following traffic-processor pods:

```shell
traffic-processor-foo-apse2-internal-864d8ffglst8   1/1     Running            0                 3d19h
traffic-processor-foo-apse2-prometheus-0            2/2     Running            0                 17d
traffic-processor-foo-apse2-proxy-668866769-fznwh   1/1     Running            0                 4d19h
traffic-processor-foo-apse2-proxy-668866769-zvsnl   1/1     Running            0                 4d18h
traffic-processor-bar-alertmanager-0                   1/1     Running            0                 11d
traffic-processor-bar-fluentd-86bfb8fbc4-4qb5h         1/1     Running            0                 11d
traffic-processor-bar-fluentd-86bfb8fbc4-jxfrd         1/1     Running            0                 5d7h
traffic-processor-bar-internal-5c89c88cbd-8llt9        1/1     Running            0                 2d21h
traffic-processor-bar-internal-5c89c88cbd-wh8v4        1/1     Running            0                 3d9h
traffic-processor-bar-prometheus-0                     2/2     Running            0                 11d
traffic-processor-bar-proxy-57dc88899c-clxr4           1/1     Running            0                 3d9h
traffic-processor-bar-proxy-57dc88899c-j5vgn           1/1     Running            0                 3d9h
```

The program will output the following:
```
-------------------------------------------------------------------------------------
          tenant           | Total | Pending | Running | Failed | Unknown | Succeeded
-------------------------------------------------------------------------------------
foo                        | 2     | 0       | 2       | 0      | 0       | 0
bar                        | 2     | 0       | 2       | 0      | 0       | 0
-------------------------------------------------------------------------------------
```

For efficiency, each tenant is processed in parallel using thread in Rust.
