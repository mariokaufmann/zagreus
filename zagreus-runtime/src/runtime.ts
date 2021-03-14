import {TemplateWebsocket} from './websocket/template-websocket';
import {WebsocketSender} from './websocket/websocket-sender';
import {WebsocketHandler} from './websocket/websocket-handler';
import {installErrorHandler} from './error';

function runWebsocket():void {
    const locationPathName = window.location.pathname;
    const pathname = locationPathName.endsWith('/') ? locationPathName.substring(0, locationPathName.length - 1) : locationPathName;
    const templateName = pathname.substring(locationPathName.lastIndexOf('/') + 1);
    const websocket = new TemplateWebsocket(`ws://${window.location.host}/ws/template/${templateName}`);
    const websocketSender = new WebsocketSender(websocket);

    installErrorHandler(websocketSender);

    websocket.messageHandler = new WebsocketHandler(websocketSender);

    websocket.run();
}

runWebsocket();
