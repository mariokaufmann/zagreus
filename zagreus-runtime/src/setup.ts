import {
  getInternalZagreusState,
  ZagreusContainerSetupArguments,
  ZagreusSetupArguments,
} from "./runtime";
import { runWebsocket } from "./websocket/run";
import { getZagreusElement } from "./utils";
import {
  applyAnimation,
  getMaxTimeoutFromSequences,
} from "./manipulation/animation";
import { removeClassOnElement } from "./manipulation/css";

const ZagreusHiddenClassName = "zagreus-hidden";

function setupContainer(args: ZagreusContainerSetupArguments) {
  const container = getZagreusElement(args.name);

  document.body.style.backgroundColor = "transparent";

  // add zagreus hidden class
  const css = `.${ZagreusHiddenClassName} { visibility: hidden }`;
  const style = document.createElement("style");
  style.appendChild(document.createTextNode(css));
  document.head.appendChild(style);

  container.style.width = `${args.width}px`;
  container.style.height = `${args.height}px`;
}

export function setup(args: ZagreusSetupArguments) {
  const state = getInternalZagreusState();

  state.instance = args.instance;
  state.host = args.host;
  state.port = args.port;

  // TODO make type in setup args a different type with nullable properties
  if (args.animationSequences) {
    state.animationsSequences = args.animationSequences.map((sequence) => ({
      ...sequence,
      steps: sequence.steps.map((step) => ({
        ...step,
        animations: step.animations.map((animation) => ({
          direction: "normal",
          iterations: 1,
          ...animation,
        })),
      })),
    }));
  }

  setupContainer(args.container);

  // run initial animations
  const onLoadAnimationSequences = state.animationsSequences
    .filter((sequence) => sequence.onLoad)
    .map((sequence) => sequence.name);
  onLoadAnimationSequences.forEach((sequence) => applyAnimation(sequence));
  const maxTimeout = getMaxTimeoutFromSequences(onLoadAnimationSequences);
  setTimeout(() => {
    removeClassOnElement(args.container.name, ZagreusHiddenClassName);
    runWebsocket();
  }, maxTimeout);
}
