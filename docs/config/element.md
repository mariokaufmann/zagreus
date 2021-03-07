# Element config
The file _elements.yaml_ in a zagreus template project can be used configure elements in your template.

Configuration elements:
- id: the id (assigned in the SVG editor) of the element that should be configured
- align: the alignment config of the element. It specifies if and how the element should be aligned to itself or other elements when changing its contents

## Alignment config
The alignment config for an element has the following possible configuration options:
- horizontal: how the element should be horizontally aligned. It can be `center`, `left` or `right`
- vertical: how the element should be vertically aligned. It can be `center`, `top` or `bottom`
- with: which other element this element should be aligned with

The `with` property only needs to be specified if either of the alignment properties are set to `center`.

## Example
```yaml
---
elements:
  - id: ScoreboardTimeText
    align:
      horizontal: center
      with: ScoreboardTimeBackground
  - id: ScoreboardAwayTeamText
    align:
      horizontal: right
  - id: ScoreboardLogoImage
    align:
      horizontal: center
      vertical: center
      with: ScoreboardLogoBackground
```
This example defines the element config for three elements. `ScoreboardTimeText` is horizontally centered in `ScoreboardTimeBackground`. `ScoreboardAwayTeamText` is right-aligned (which means its right edge will always stay in the same place when changing its content).
Finally, `ScoreboardLogoImage` is center aligned both vertically and horizontally in `ScoreboardLogoBackground`.
