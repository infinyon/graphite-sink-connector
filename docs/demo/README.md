Getting all Non-Null Values

You can use Graphite's Render API to inspect on stored values with a JSON value
format output.

```bash
curl http://localhost:12345/render\?target\=weather.temperature.ca.sandiego\&format\=json\&noNullPoints
```

```json
[
  {
    "target": "weather.temperature.ca.sandiego",
    "tags": {
      "name": "weather.temperature.ca.sandiego"
    },
    "datapoints": [
      [146.5, 1688757960],
      [126.2, 1688758020],
      [38.5, 1688758080]
    ]
  }
]
```
