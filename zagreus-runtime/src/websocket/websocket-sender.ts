import { TemplateWebsocket } from "./template-websocket";
import { ClientMessage, StateSetPayload, TaggedEnumType } from "./types";

export class WebsocketSender {
  constructor(private websocket: TemplateWebsocket) {}

  sendMessage(message: TaggedEnumType<ClientMessage>): void {
    if (this.websocket.isOpen()) {
      this.websocket.sendMessage(message);
    } else {
      console.error(
        `Cannot send message ${message.tag} as websocket is not open.`,
      );
    }
  }

  sendStateSetMessage(name: string, value: string): void {
    const message: TaggedEnumType<ClientMessage, StateSetPayload> = {
      tag: "StateSet",
      payload: {
        name,
        value,
      },
    };
    this.sendMessage(message);
  }
}
