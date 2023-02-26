# Changelog

## Unreleased
* Refactor architecture with many breaking changes:
  * Remove the 'template' concept. Users can now create their completely independent templates and just use zagreus as a library to hook into their templates. This allows them to create much more flexible templates with the whole span of web technologies. It is still possible to use SVG files as within the templates but a template does not revolve around a single SVG file anymore.

## 0.0.7
* Switch log file format to JSON logging

## 0.0.6
* Update multer dependency to allow API clients to upload templates in multipart form data with quoted or unquoted filenames. Previously, only quoted worked.

## 0.0.5

* Add endpoints for managing assets via the HTTP API. See updated OpenAPI spec for details: [PR #58](https://github.com/mariokaufmann/zagreus/pull/59) and [PR #65](https://github.com/mariokaufmann/zagreus/pull/65)
* Make server port configurable: [PR #64](https://github.com/mariokaufmann/zagreus/pull/64)
* Add command line arguments for starting the server to override the data folder and server port specified in the configuration file: [PR #66](https://github.com/mariokaufmann/zagreus/pull/66)
