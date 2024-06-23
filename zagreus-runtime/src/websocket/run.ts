import { TemplateWebsocket } from "./template-websocket";
import { WebsocketSender } from "./websocket-sender";
import { installErrorHandler } from "../error";
import { WebsocketHandler } from "./websocket-handler";
import { getInternalZagreusState, getWebsocketProtocol } from "../runtime";

export function runWebsocket(): void {
  const state = getInternalZagreusState();
  const websocketProtocol = getWebsocketProtocol();
  const url = `${websocketProtocol}://${state.host}:${state.port}/ws/instance/${state.instance}`;
  const websocket = new TemplateWebsocket(url);
  const websocketSender = new WebsocketSender(websocket);

  installErrorHandler(websocketSender);

  websocket.messageHandler = new WebsocketHandler(websocketSender);

  websocket.run();
}
