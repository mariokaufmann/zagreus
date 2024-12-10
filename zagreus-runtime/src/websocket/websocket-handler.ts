import { WebsocketSender } from "./websocket-sender";
import {
  EnumTypeHandler,
  ExecuteAnimationPayload,
  ManipulateClassPayload,
  SetCustomVariablePayload,
  SetImageSourcePayload,
  SetTextPayload,
  TaggedEnumType,
  TemplateMessage,
} from "./types";
import { setTextOnElement } from "../manipulation/text";
import { addClassOnElement, removeClassOnElement } from "../manipulation/css";
import { applyAnimation } from "../manipulation/animation";
import { setImageSource } from "../manipulation/image";
import { setCustomVariable } from "../manipulation/custom-variable";

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
    applyAnimation(payload.animationSequence, payload.queueId);
  },
  SetImageSource: (payload: SetImageSourcePayload) => {
    setImageSource(payload.id, payload.asset, payload.assetSource);
  },
  SetCustomVariable: (payload: SetCustomVariablePayload) => {
    setCustomVariable(payload.name, payload.value);
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
      this.sender,
    );
  }
}
