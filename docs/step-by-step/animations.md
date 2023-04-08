# Animation
We want to be able to animate the scoreboard. More precisely we want to show and hide the scoreboard by letting it fly
in and out from the upper border of the screen.

## Add animation

We can create a stylesheet containing some animations and can then connect them to elements in the animations config.

Create a file _animations.css_ in the template folder and add the following snippet to it:

```css
@keyframes moveScoreboardDown {
    from {
        transform: translateY(-300px);
    }
    to {
        transform: translateY(0);
    }
}

@keyframes moveScoreboardTimeDown {
    from {
        transform: translateY(-40px);
    }
    to {
        transform: translateY(0);
    }
}
```

This defines two animations that will move an element down by the specified amount of pixels.
We now have to connect this animation definition to a specific element by registering the animations with zagreus.

Open the file `index.html` and change the `setup` call to look like the following snippet:

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
      },
      {
        name: "ScoreboardHide",
        onLoad: true,
        steps: [
          {
            duration: 500,
            animations: [
              {
                id: "Scoreboard",
                name: "moveScoreboardDown",
                direction: "reverse"
              }
            ]
          },
          {
            duration: 10,
            animations: [
              {
                id: "ScoreboardTime",
                name: "moveScoreboardTimeDown",
                direction: "reverse"
              }
            ]
          }
        ]
      }
    ]
  });
</script>
```
Additionally, add the `animations.css` file to our html as well:

```html
<head>
  <meta charset="utf-8" />
  <title>zagreus template</title>
  <link rel="stylesheet" href="main.css" />
  <link rel="stylesheet" href="animations.css" />
</head>
```

With this we define two animation sequences, one for showing the scoreboard and one for hiding it. A sequence is made
up of one or more steps that are executed one after the other in the order they are defined. Refresh the template in your browser.
For more detailed information on how to configure animations check out the [animation documentation](../config/animation.md).

## Execute animation

To see our animation sequence in action we can execute it from the API documentation (read the chapter about dynamic text if you haven't already). To execute, use the _data/animation/_ endpoint. Try executing the _ScoreboardShow_ animation. You
should see the scoreboard fly in from the top. Now execute the _ScoreboardHide_ animation sequence. The scoreboard should disappear again. Since our show animation sequence consists of two steps the scoreboard will first fly in and
then the element displaying the time will move once the first movement is finished.

## On load animation

We designed the scoreboard in the position in which it will end up when it is shown on the screen. However, we want the
scoreboard to be initially hidden when the template is first loaded. It would be very inconvenient if we had to design
the scoreboard in its hidden position. Zagreus allows us to configure for each animation sequence whether it should be played when the template initially loads. This is done before the visual elements in the template are actually visible, thus making this
a great feature for bringing all template elements into place before the template finishes loading.
For our example animation `ScoreboardHide` we specified the property `onLoad` to be be _true_, which means that it will automatically be played on template load.
```
 {
        name: "ScoreboardHide",
        onLoad: true,
        steps: [
          {
            duration: 500,
            animations: [
              {
                id: "Scoreboard",
                name: "moveScoreboardDown",
                direction: "reverse"
              }
            ]
          },
          {
            duration: 10,
            animations: [
              {
                id: "ScoreboardTime",
                name: "moveScoreboardTimeDown",
                direction: "reverse"
              }
            ]
          }
        ]
      }
```

We have now explored all the main functions of zagreus. Have a look at the chapter about [playout](./playout.md) for how to actually use the template we just created as a graphics overlay.