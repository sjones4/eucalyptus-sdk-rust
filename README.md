## Eucalyptus SDK for Rust

A Eucalyptus SDK for Rust using [Rusoto](https://github.com/rusoto/rusoto)

### Generating Service APIs

On update of the JSON models service code should be regenerated using the rusoto `service_crategen`
and the models / `services.json` from this repository.

### Usage

See the example `euca` client for sdk usage:

    # euca services describe-types
    SVCTYPE	autoscaling	false	Auto Scaling API service
    SVCTYPE	cluster	false	The Cluster Controller service
    SVCTYPE	dns	false	Eucalyptus DNS server
    SVCTYPE	tokens	false	STS API service
    ...

The example client demonstrates the properties and service management
services.
