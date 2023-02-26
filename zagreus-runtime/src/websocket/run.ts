import { TemplateWebsocket } from "./template-websocket";
import { WebsocketSender } from "./websocket-sender";
import { installErrorHandler } from "../error";
import { WebsocketHandler } from "./websocket-handler";
import { getInternalZagreusState } from "../runtime";

export function runWebsocket(): void {
  const state = getInternalZagreusState();
  const url = `ws://${state.host}:${state.port}/ws/instance/${state.instance}`;
  console.log(url);
  const websocket = new TemplateWebsocket(url);
  const websocketSender = new WebsocketSender(websocket);

  installErrorHandler(websocketSender);

  websocket.messageHandler = new WebsocketHandler(websocketSender);

  websocket.run();
}
