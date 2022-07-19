# Bowtie

[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](https://github.com/json-schema-org/.github/blob/main/CODE_OF_CONDUCT.md)
[![Project Status: WIP – Initial development is in progress, but there has not yet been a stable, usable release suitable for the public.](https://www.repostatus.org/badges/latest/wip.svg)](https://www.repostatus.org/#wip)

Bowtie is a *meta*-validator of the [JSON Schema specification](https://json-schema.org/), by which we mean it coordinates executing *other* [validator implementations](https://json-schema.org/implementations.html), collecting and reporting on their results.

To do so it defines a simple input/output protocol (specified [alongside this README](./io-schema.json)) which validator implementations can implement, and it provides a CLI which can execute supported implementations.

It's called Bowtie because it fans in lots of JSON then fans out lots of results: `>·<`. Looks like a bowtie, no?
Also because it's elegant – we hope.

## Execution

In general, executing `bowtie` consists of providing 2 pieces of input:

* The names of one or more supported implementations to execute
* One or more test cases to run against these implementations (schemas, instances and optionally, expected validation results)

Given these, `bowtie` will report on the result of executing each implementation against the input schema/instance pairs.
If expected results are provided, it will compare the results produced against the expected ones, reporting on any implementations which differ from the expected output.

### CLI

A sample invocation of the CLI is:

```sh
$ bowtie run --all <<EOF
{"description": "stuff", "schema": {}, "tests": [{"description": "a test", "instance": {"foo": "bar"}}] }
EOF
{"valid": true}
```

(TODO)

## Uses

A key use of `bowtie` is in executing as input the [official test suite](https://github.com/json-schema-org/JSON-Schema-Test-Suite) and comparing the results produced by implementations to the expected ones from the suite.

Of course one isn't limited to just the test cases in the test suite, as `bowtie` can be used to compare the validation results of any input across its supported implementations.

## Adding an Implementation

(TODO)
