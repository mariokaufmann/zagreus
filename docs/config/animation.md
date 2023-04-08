# Animation config
When setting up a template we can pass _animation sequences_ to zagreus. This registers animation sequences with zagreus which makes it possible to then later play these animations on your overlay.

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
- id: the id of the element the animation should be applied to. This corresponds to the `data-zag` attribute in the HTML
- name: the name of the CSS animation definition that should be applied to the element. Example: for a name of `moveLowerThirdUp` zagreus expects a CSS animation definition such as for example
```css
@keyframes moveLowerThirdUp {
    from { transform: translateY(-100px); }
    to { transform: translateY(0); }
}
```
- direction: Indicates in which direction the animation should be run. This can be any of the following values:
  - `normal`: the animation will run forwards
  - `reverse`: the animation will run backwards
  - `alternate`: the animation will first run forwards and then backwards if it is run more than once (see `iterations`)
  - `alternate-reverse`: the animation will first run backwards and then forwards if it is run more than once (see `iterations`)
    
- iterations: how many times the animation should be run. Can either be natural number (e.g. `7`) or `infinite` indicating that the animation will repeat forever
    
## On load
For each animation sequence it is possible to specify whether it should play automatically when the template loads. This can be used to bring the template into a useful initial state.

## Example
```html
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
```html
<script>
  window.zagreus.setup({
    host: "localhost",
    port: 58180,
    instance: 'test-template',
    container: {
      name: 'zagreus-container',
      width: 1920,
      height: 1080,
    },
    animationSequences: [
      {
        name: "ScoreboardShow",
        onLoad: true,
        steps: [
          {
            duration: 500,
            animations: [
              {
                id: "Scoreboard",
                name: "moveScoreboardDown"
              }
            ]
          },
          {
            duration: 250,
            animations: [
              {
                id: "ScoreboardTime",
                name: "moveScoreboardTimeDown"
              }
            ]
          }
        ]
      }
    ]
  });
</script>
```
In this example we define an animation sequence with the name `ScoreboardShow`. It consists of two steps. The first one starts immediately and ends after 500 milliseconds. The second one starts after the first one has finished (so after 500) milliseconds and ends after another 250 milliseconds. Both animation steps apply an animation to a single element in the template.
Furthermore, because we specified `onLoad: true` for the sequence it will be executed once the template loads.
