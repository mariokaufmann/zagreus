import { AnimationSequence } from "./websocket/types";
import { registerAnimations, setup } from "./setup";

declare global {
  interface Window {
    zagreus: ZagreusState;
  }
}

export type ErrorReporter = (error: Error) => void;

export interface ZagreusContainerSetupArguments {
  name: string;
  width: number;
  height: number;
}

export interface ZagreusSetupArguments {
  host: string;
  port: string;
  instance: string;
  container: ZagreusContainerSetupArguments;
  animationSequences?: AnimationSequence[];
}

export interface ZagreusState {
  setup: (args: ZagreusSetupArguments) => void;
  registerAnimations: (...animation: AnimationSequence[]) => void;
  _internal: InternalZagreusState;
}

export interface InternalZagreusState {
  instance: string;
  host: string;
  port: string;
  animationSequences: Record<string, AnimationSequence>;
  errorReporter: ErrorReporter;
}

if (!window.zagreus) {
  window.zagreus = {
    setup: setup,
    registerAnimations: registerAnimations,
    _internal: {
      instance: undefined,
      host: undefined,
      port: undefined,
      animationSequences: {},
      errorReporter: undefined,
    },
  };
}

export const getInternalZagreusState = (): InternalZagreusState => {
  return getZagreusState()._internal;
};

export const getZagreusState = (): ZagreusState => {
  return window.zagreus;
};

export const getUrlOnServer = (path: string): string => {
  const state = getInternalZagreusState();
  return `http://${state.host}:${state.port}${path}`;
};

// have setup function that takes -server url, template width & height, animation sequences
// have two sources for assets: template and zagreus
// TODO port animation validation logic from generator to here
