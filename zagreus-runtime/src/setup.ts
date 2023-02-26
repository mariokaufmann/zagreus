import {
  getInternalZagreusState,
  ZagreusContainerSetupArguments,
  ZagreusSetupArguments,
} from "./runtime";
import { runWebsocket } from "./websocket/run";
import { getZagreusElement } from "./utils";

function setupContainer(args: ZagreusContainerSetupArguments) {
  const container = getZagreusElement(args.name);

  document.body.style.backgroundColor = "transparent";
  container.style.width = `${args.width}px`;
  container.style.height = `${args.height}px`;

  // run initial animations
  // TODO

  // remove hidden class
  // TODO
}

export function setup(args: ZagreusSetupArguments) {
  const state = getInternalZagreusState();

  state.instance = args.instance;
  state.host = args.host;
  state.port = args.port;
  state.animationsSequences = args.animationSequences ?? [];

  setupContainer(args.container);

  // start
  runWebsocket();
}
