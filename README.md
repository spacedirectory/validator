# Validation server for SpaceAPI endpoints

Written in Python 3 with [Bottle](http://bottlepy.org/).

https://validator.spacedirectory.org/v1/

[![Travis CI][travis-ci-badge]][travis-ci]


# Dev setup

## Dependencies

Install Python 3 and [pipenv](https://github.com/pypa/pipenv).

Then:

    pipenv install --dev

Enable env:

    pipenv shell

(Or prefix every Python command with `pipenv run`.)

## Starting the Server

Start the server:

    cd validator
    python server.py

## Testing

To run tests:

    cd validator
    py.test -v


# API

## Request

To send a validation request, send a POST request to `/v1/validate/` with
`Content-Type: application/json`. The payload (in JSON format) should look like
this:

```javascript
{
    "data": "..."
}
```

The `data` field should contain the SpaceAPI endpoint data as a JSON string.

Example (curl):

    curl \
        -X POST \
        -H "Content-Type: application/json" \
        https://validator.spacedirectory.org/v1/validate/ \
        -d'{"data": "{\"api\": \"0.13\"}"}'

Example (httpie):

    http POST \
        https://validator.spacedirectory.org/v1/validate/ \
        data='{"api": "0.13"}'

## Response

If the request is not malformed, the endpoint returns a HTTP 200 response with
`Content-Type: application/json`.

The success response looks like this:

```javascript
{
    "valid": true,
    "message": null
}
```

The error response looks like this:

```javascript
{
    "valid": false,
    "message": "Error details"
}
```

It is planned that more error details (like row/col) will be added in the future.


# License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

<!-- Badges -->
[travis-ci]: https://travis-ci.org/SpaceApi/validator
[travis-ci-badge]: https://img.shields.io/travis/SpaceApi/validator/master.svg
