import { AnimationSequence } from "./websocket/types";
import { registerAnimations, setup } from "./setup";
import { AnimationQueue } from "./manipulation/animation";
import { WebsocketSender } from "./websocket/websocket-sender";

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
  secure: boolean;
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
  secure?: boolean;
  websocketSender?: WebsocketSender,
  animationSequences: Record<string, AnimationSequence>;
  animationQueues: Record<string, AnimationQueue>;
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
      secure: false,
      websocketSender: undefined,
      animationSequences: {},
      animationQueues: {},
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
  const httpProtocol = getHttpProtocol();
  return `${httpProtocol}://${state.host}:${state.port}${path}`;
};

export const getHttpProtocol = (): string => {
  return getInternalZagreusState().secure ? "https" : "http";
};

export const getWebsocketProtocol = (): string => {
  return getInternalZagreusState().secure ? "wss" : "ws";
};
