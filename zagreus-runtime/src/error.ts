import { WebsocketSender } from "./websocket/websocket-sender";
import {
  LogErrorPayload,
  TaggedEnumType,
  TemplateMessage,
} from "./websocket/types";
import { ErrorReporter, getInternalZagreusState } from "./runtime";

export const installErrorHandler = (websocketSender: WebsocketSender): void => {
  window.addEventListener(
    "error",
    (evt) => {
      let error = <Error>evt.error;
      if (!error) {
        const target = <HTMLElement>evt.target;
        error = {
          name: "Anonymous error",
          stack: "",
          message: `Error occurred on element ${target.id ?? target.tagName}`,
        };
      }
      reportErrorOnSender(websocketSender, error);
    },
    {
      capture: true,
    }
  );
  getInternalZagreusState().errorReporter = getErrorReporter(websocketSender);
};

export const reportErrorMessage = (message: string): void => {
  const error: Error = {
    name: "zagreus error",
    message,
    stack: "",
  };
  getInternalZagreusState().errorReporter(error);
};

const getErrorReporter = (websocketSender: WebsocketSender): ErrorReporter => {
  return (error) => reportErrorOnSender(websocketSender, error);
};

const reportErrorOnSender = (
  websocketSender: WebsocketSender,
  error: Error
): void => {
  console.error(error);
  const message: TaggedEnumType<TemplateMessage, LogErrorPayload> = {
    tag: "LogError",
    payload: {
      stack: error.stack,
      message: error.message,
    },
  };
  websocketSender.sendMessage(message);
};
