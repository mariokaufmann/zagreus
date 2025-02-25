import { TemplateWebsocket } from "./template-websocket";
import { TaggedEnumType, TemplateMessage } from "./types";

export class WebsocketSender {
  constructor(private websocket: TemplateWebsocket) {
  }

  sendMessage(message: TaggedEnumType<TemplateMessage>): void {
    if (this.websocket.isOpen()) {
      this.websocket.sendMessage(message);
    } else {
      console.error(
        `Cannot send message ${message.tag} as websocket is not open.`
      );
    }
  }

  sendAnimationCompletedMessage(queueName: string, animationName: string): void {
    this.sendMessage({
      tag: "ExecuteAnimation", payload: {
        queue: queueName,
        animation: animationName
      }
    });
  }
}
