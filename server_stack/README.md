## Setting up InfluxDB & Telegraf

Start the docker container:

```
make influxdb/up
```

Create a bucket

Create an access token

Rename secrets_template.env to secrets.env and set the INFLUX_TOKEN to the value of the token

Restart the stack:
```
make stack/restart
```



