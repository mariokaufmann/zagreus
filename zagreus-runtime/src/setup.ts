import {
  getInternalZagreusState,
  StateChangeListener,
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
import { AnimationSequence } from "./websocket/types";

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
  state.secure = args.secure;

  setupContainer(args.container);
  if (args.animationSequences) {
    registerAnimations(...args.animationSequences);
  }

  // run initial animations after a timeout (to allow time for registering other animations)
  setTimeout(() => {
    const onLoadAnimationSequences = Object.entries(state.animationSequences)
      .filter(([name, sequence]) => sequence.onLoad)
      .map(([name, sequence]) => sequence.name);
    onLoadAnimationSequences.forEach((sequence) =>
      applyAnimation(sequence, undefined),
    );
    const maxTimeout = getMaxTimeoutFromSequences(onLoadAnimationSequences);
    setTimeout(() => {
      removeClassOnElement(args.container.name, ZagreusHiddenClassName);
      runWebsocket();
    }, maxTimeout);
  }, 100);
}

export function registerAnimations(...animations: AnimationSequence[]) {
  // TODO make type in setup args a different type with nullable properties
  const state = getInternalZagreusState();
  animations
    .map((sequence) => ({
      ...sequence,
      steps: sequence.steps.map((step) => ({
        start: 0,
        ...step,
        animations: step.animations.map((animation) => ({
          direction: "normal",
          iterations: 1,
          ...animation,
        })),
      })),
    }))
    .forEach(
      (sequence) => (state.animationSequences[sequence.name] = sequence),
    );
}

export function registerStateListener(
  stateName: string,
  listener: StateChangeListener,
) {
  const state = getInternalZagreusState();
  if (!state.states[stateName]) {
    state.states[stateName] = { value: "", changedListeners: [] };
  }
  state.states[stateName].changedListeners.push(listener);
}
