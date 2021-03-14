import {WebsocketSender} from './websocket/websocket-sender';
import {LogErrorPayload, TaggedEnumType, TemplateMessage} from './websocket/types';

export const installErrorHandler = (websocketSender: WebsocketSender) => {
    window.addEventListener('error', evt => {
        const error = <Error>evt.error;
        const message: TaggedEnumType<TemplateMessage, LogErrorPayload> = {
            tag: 'LogError',
            payload: {
                stack: error.stack,
                message: error.message,
            },
        };
        websocketSender.sendMessage(message);
    }, {
        capture: true,
    });
};
