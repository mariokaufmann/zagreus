# Animation config
The file _animations.yaml_ in a zagreus template project can be used to define and configure animation sequences for your template.

## Sequences
Animations for zagreus templates are defined through animation sequences. An animation sequence consists of:
- name: the name of an animation sequence should be unique within the template
- steps: one or more animations steps (see below)

### Step
An animation step is one part of an animation sequence. It consists of:
- duration: the duration of the step in milliseconds
- start: the start time (in milliseconds) of the animation step from the beginning of the animation sequence
- animations: one or more _element animations_ (see below) that should be applied in this animation step

In case the start time of a step is specified the duration value will be ignored. If no start value is specified the step will start once the steps before it have completed animating.

### Element animation
An element animation defines how a CSS animation is defined to a specific element. It consists of:
- id: the id (assigned in the SVG editor) of the element the animation should be applied to
- name: the name of the CSS animation definition that should be applied to the element. Example: for a name of `moveLowerThirdUp` zagreus expects a CSS animation definition such as for example
```css
@keyframes moveLowerThirdUp {
    from { transform: translateY(-100px); }
    to { transform: translateY(0); }
}
```
- direction: either `normal` or `reverse`. Indicates whether the animation should be played forwards or backwards

## On load
Under `onLoad -> animationSequences` it is possible to list animation sequences that should be executed when the template loads. This can be used to bring the template into a useful initial state.

## Example
```yaml
---
onLoad:
  animationSequences:
    - "ScoreboardShow"
sequences:
  - name: ScoreboardShow
    steps:
      - duration: 500
        animations:
          - id: Scoreboard
            name: moveScoreboardDown
      - duration: 250
        animations:
          - id: ScoreboardTime
            name: moveScoreboardTimeDown

```
In this example we define an animation sequence with the name `ScoreboardShow`. It consists of two steps. The first one starts immediately and ends after 500 milliseconds. The second one starts after the first one has finished (so after 500) milliseconds and ends after another 250 milliseconds. Both animation steps apply an animation to a single element in the template.
Furthermore, because the animation is listed under the `onLoad` animations it will be executed once the template loads.
