import { WebsocketSender } from "./websocket-sender";
import {
  EnumTypeHandler,
  ExecuteAnimationPayload,
  ManipulateClassPayload,
  SetImageSourcePayload,
  SetTextPayload,
  TaggedEnumType,
  TemplateMessage,
} from "./types";
import { setTextOnElement } from "../manipulation/text";
import { addClassOnElement, removeClassOnElement } from "../manipulation/css";
import { applyAnimation } from "../manipulation/animation";
import { setImageSource } from "../manipulation/image";

const templateMessageHandlers: EnumTypeHandler<
  TemplateMessage,
  WebsocketSender
> = {
  SetText: (payload: SetTextPayload) => {
    setTextOnElement(payload.id, payload.text);
  },
  AddClass: (payload: ManipulateClassPayload) => {
    addClassOnElement(payload.id, payload.class);
  },
  RemoveClass: (payload: ManipulateClassPayload) => {
    removeClassOnElement(payload.id, payload.class);
  },
  ExecuteAnimation: (payload: ExecuteAnimationPayload) => {
    applyAnimation(payload.animationSequence);
  },
  SetImageSource: (payload: SetImageSourcePayload) => {
    setImageSource(payload.id, payload.asset, payload.assetSource);
  },
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  LogError: () => {},
};

export class WebsocketHandler {
  constructor(private sender: WebsocketSender) {}

  handleMessage(message: string): void {
    const parsedMessage: TaggedEnumType<TemplateMessage> = JSON.parse(message);
    templateMessageHandlers[parsedMessage.tag](
      parsedMessage.payload,
      this.sender
    );
  }
}
