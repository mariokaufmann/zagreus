import { WebsocketHandler } from "./websocket-handler";
import { ClientMessage, TaggedEnumType } from "./types";

const reconnectionTimeoutMillis = 2000;

export class TemplateWebsocket {
  messageHandler: WebsocketHandler | undefined = undefined;

  private websocket: WebSocket | undefined;
  private wasOpen = false;

  constructor(private url: string) {}

  run(): void {
    this.handleStateChange();
  }

  isOpen(): boolean {
    if (!this.websocket) {
      return false;
    }
    return this.websocket.readyState === this.websocket.OPEN;
  }

  sendMessage(message: TaggedEnumType<ClientMessage>): void {
    const serializedMessage = JSON.stringify(message);
    if (this.websocket) {
      this.websocket.send(serializedMessage);
    }
  }

  private handleStateChange(): void {
    if (!this.websocket) {
      this.websocket = new WebSocket(this.url);
      this.websocket.onmessage = (event) => this.onMessage(event.data);
      this.websocket.onopen = () => {
        this.onOpen();
        this.handleStateChange();
      };
      this.websocket.onclose = (event) => {
        this.onClose(event);
        this.handleStateChange();
      };
      this.websocket.onerror = () => TemplateWebsocket.onError();
      return;
    }

    switch (this.websocket.readyState) {
      case WebSocket.OPEN:
        break;
      case WebSocket.CLOSED:
        this.websocket = undefined;
        setTimeout(() => this.handleStateChange(), reconnectionTimeoutMillis);
        break;
    }
  }

  private onOpen(): void {
    this.wasOpen = true;
  }

  private static onError(): void {
    console.error("Error on websocket.");
  }

  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  private onClose(event: CloseEvent): void {
    if (this.wasOpen) {
      this.wasOpen = false;
    }
  }

  private onMessage(message: string): void {
    if (this.messageHandler) {
      this.messageHandler.handleMessage(message);
    }
  }
}
