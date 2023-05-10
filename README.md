# monerod-exporter

Serve monerod metrics under `/metrics` path.

Forked from [bitcoin-exporter](https://github.com/eburghar/bitcoin-exporter).
Metrics definitions from [monerod-exporter](https://github.com/hundehausen/monerod-exporter).

```
monerod-exporter 0.0.1

Usage: monerod-exporter [-c <config>] [-v]

Export monerod metrics to prometheus format

Options:
  -c, --config      configuration file
  -v, --verbose     more detailed output
  --help            display usage information
```

The configuration accepts the following keys. `host` and `bind` are optional. `user`, `password` and `host` represent
the monerod rpc parameters.

```yaml
user: user
password: changeme
host: 'http://localhost:8332'
bind: '127.0.0.1:9898'
```

