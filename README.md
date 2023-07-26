# SchemaDoc

## 🚧🚧🚧 WIP (work-in-progress) 🚧🚧🚧

SchemaDoc is an open-source project that allows you to compare OpenAPI schemas and visualize the results in a
Swagger-like manner. It provides a convenient way to track changes between different versions of OpenAPI schemas and
identify breaking changes.

![SchemaDoc Project View](assets/screenshot.png)

## Features

- **Project and Versions**: You can create projects and manage multiple versions of OpenAPI schemas within each project.
  This enables you to track changes over time and compare different versions easily.
- **Data Sources**: SchemaDoc supports configuring data sources to pull OpenAPI schemas from. Currently, it only
  supports
  basic URL GET requests. You can specify the URL from which the schema should be fetched.
- **Scheduled Data Pulling**: SchemaDoc allows you to schedule the pulling of OpenAPI schemas from the configured data
  sources. By default, it pulls the data every 5 minutes, ensuring that you always have the most up-to-date information.
- **Breaking Changes**: SchemaDoc calculates breaking changes between different versions of OpenAPI schemas. It helps
  you
  identify modifications that may cause compatibility issues with existing clients.
- **Alerts**: You can configure alerts to receive notifications about schema changes. SchemaDoc supports sending summary
  alerts to Slack or Google Chat. There are two kinds of alerts available:
    - **all**: Send an alert for any change detected in the schema.
    - **breaking**: Send an alert only if there are breaking changes in the schema.
- **File-based Storage**: SchemaDoc does not require a database. It stores all data in files, making it easy to set up
  and
  deploy.
- **Dependencies**: SchemaDoc supports specifying dependencies between projects. This allows you to track how changes in
  one project may impact the other.

## Configuration (schemadoc.yaml)

The configuration file for SchemaDoc (schemadoc.yaml) follows the structure below:

```yaml
version: "0.1"

data:
  stripe:
    slug: stripe
    name: Strip API
    kind: server
    description: empty
    alerts:
      - name: Slack breaking
        kind: breaking
        source: own
        is_active: true
        service: Slack
        service_config:
          hook: https://hooks.slack.com/services/ABCDEFGHIJK/123456789/A1B2C3D4e5f6

    data_source:
      name: Stripe Github raw
      source: !Url { url: https://raw.githubusercontent.com/stripe/openapi/master/openapi/spec3.json }

    dependencies:
      - project: petstore # Change me or remove
        version: 0
```

In this configuration file, you can define multiple projects under the data section. Each project has a unique slug,
name, and description. You can configure alerts for each project, specifying their name, kind, source, and other related
information.

The **data_source** section allows you to configure the data source from which the OpenAPI schema will be pulled.
Currently, only basic HTTP GET requests are supported. You can provide the name and URL of the data source.

There are two kinds of projects `server` and `client`, they differ only visually on UI. Client does not have
versions and overview page shows the client dependencies.

## Getting Started

#### TODO

## Contributions

SchemaDoc is an open-source project, and contributions are welcome. If you have any ideas, suggestions, or bug reports,
please feel free to submit them to the project's repository.

## License

SchemaDoc is released under the Apache 2.0 License.