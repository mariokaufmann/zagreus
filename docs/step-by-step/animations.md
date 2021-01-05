# Animation

We want to be able to animate the scoreboard. More precisely we want to show and hide the scoreboard by letting it fly
in and out from the upper border of the screen.

# Add animation

We can create a stylesheet containing some animations and can then connect them to elements in the animations config.

Create a file _animations.css_ in the folder _asset_ and add the following snippet to it:

```css
@keyframes moveScoreboardDown {
    from {
        transform: translateY(-400px);
    }
    to {
        transform: translateY(0);
    }
}

@keyframes moveScoreboardTimeDown {
    from {
        transform: translateY(-100px);
    }
    to {
        transform: translateY(0);
    }
}
```

This defines two animations that will move the element which they are applied to down by the specified amount of pixels.
We now have to connect this animation definition to a specific element by creating animation sequences in _animations.yaml_.

Open the file _animations.yaml_ and change it so it looks like the following code:

```yaml
---
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
  - name: ScoreboardHide
    steps:
      - duration: 500
        animations:
          - id: Scoreboard
            name: moveScoreboardDown
            direction: reverse
      - duration: 10
        animations:
          - id: ScoreboardTime
            name: moveScoreboardTimeDown
            direction: reverse
```

With this we define two animation sequences, one for showing the scoreboard and one for hiding it. An sequence is made
up of one or more steps that are executed one after the other in the order they are defined. Rebuild the template and
upload it to the server (this is done automatically if you have started the generator in _watch_ mode).

# Execute animation

To see our animation sequence in action we can execute it from the server UI (TODO go to UI execute hide animation). You
should see the scoreboard disappear. Now execute the _ScoreboardShow_ animation sequence. The scoreboard should come
back flying from the top. Since our show animation sequence consists of two steps the scoreboard will first fly in and
then the element displaying the time will move once the first movement is finished.

# On load animation

We designed the scoreboard in the position in which it will end up when it is shown on the screen. However, we want the
scoreboard to be initially hidden when the template is first loaded. It would be very inconvenient if we had to design
the scoreboard in its hidden position. Zagreus allows us list animation sequences that should be played when the
template initially loads. This is done before the visual elements in the template are actually visible, thus making this
a great feature for bringing all template elements into place before the template finishes loading. Open the file _zagreus-template.yaml_ and add the _ScoreboardHide_ as a _on load_ animation like so:

```yaml
---
name: my-template
onLoad:
  animationSequences:
    - "ScoreboardHide"
devServer:
  address: localhost
  port: 4300
```

If you reload the template now the scoreboard should initially be invisible and only show up once you execute the _ScoreboardShow_ animation.