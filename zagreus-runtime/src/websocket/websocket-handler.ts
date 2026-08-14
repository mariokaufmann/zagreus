import { WebsocketSender } from "./websocket-sender";
import {
  EnumTypeHandler,
  ExecuteAnimationPayload,
  ManipulateClassPayload,
  ServerMessage,
  SetCustomVariablePayload,
  SetImageSourcePayload,
  SetStatePayload,
  SetTextPayload,
  TaggedEnumType,
} from "./types";
import { setTextOnElement } from "../manipulation/text";
import { addClassOnElement, removeClassOnElement } from "../manipulation/css";
import { applyAnimation } from "../manipulation/animation";
import { setImageSource } from "../manipulation/image";
import { setCustomVariable } from "../manipulation/custom-variable";
import { getInternalZagreusState } from "../runtime";

const templateMessageHandlers: EnumTypeHandler<ServerMessage, WebsocketSender> =
  {
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
    SetState: (payload: SetStatePayload, sender) => {
      const internalState = getInternalZagreusState();
      // The server may send a null value; treat it as the empty state value,
      const value = payload.value ?? "";
      if (!internalState.states[payload.name]) {
        internalState.states[payload.name] = {
          value,
          changedListeners: [],
        };
      } else {
        internalState.states[payload.name].value = value;
      }
      internalState.states[payload.name].changedListeners.forEach((listener) =>
        listener(value),
      );
      sender.sendStateSetMessage(payload.name, value);
    },
  };

export class WebsocketHandler {
  constructor(private sender: WebsocketSender) {}

  handleMessage(message: string): void {
    const parsedMessage: TaggedEnumType<ServerMessage> = JSON.parse(message);
    templateMessageHandlers[parsedMessage.tag](
      parsedMessage.payload,
      this.sender,
    );
  }
}
