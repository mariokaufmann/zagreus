# Template configuration

The file _elements.yaml_ in a zagreus template project can be used configure elements in your template.

Example:

```yaml
---
name: my-template
width: 1920
height: 1080
devServer:
  address: localhost
  port: 58179

```

This defines a zagreus template with the name `my-template` with a resolution of 1920px by 1080px. The `devServer`
settings are used for the zagreus generator. They determine to which server a template is uploaded when the
command `zagreus-generator upload` is invoked.