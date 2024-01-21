# traffic-processor-pods-checker

A simple program that summarizes the status of traffic-processor pods for a list of tenants

Usage: ```cargo run -- <tenant-names in comma separated list>, e.g "foo,bar"```

## Example output

```
target/release/check-traffic-processor-pods "foo,bar"
-------------------------------------------------------------------------------------
          tenant           | Total | Pending | Running | Failed | Unknown | Succeeded
-------------------------------------------------------------------------------------
foo                        | 2     | 0       | 2       | 0      | 0       | 0
bar                        | 2     | 0       | 2       | 0      | 0       | 0
-------------------------------------------------------------------------------------
```
