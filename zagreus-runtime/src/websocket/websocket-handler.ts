import {WebsocketSender} from "./websocket-sender";
import {
    EnumTypeHandler,
    ExecuteAnimationPayload,
    LoadAnimationsPayload,
    LoadElementsPayload,
    ManipulateClassPayload,
    OnLoadPayload,
    SetImageSourcePayload,
    SetTextPayload,
    TaggedEnumType,
    TemplateMessage
} from "./types";
import {setTextOnElement} from "../manipulation/text";
import {addClassOnElement, removeClassOnElement, showZagreusSvgContainer} from "../manipulation/css";
import {getZagreusState} from "../data/data";
import {applyAnimation, getMaxTimeoutFromSequences} from "../manipulation/animation";
import {setImageSource} from "../manipulation/image";
import {saveInitialAlignmentStates} from "../manipulation/manipulation";
import {flattenTransforms} from "../manipulation/transform";
import {flattenUseElements} from "../manipulation/use";

const templateMessageHandlers: EnumTypeHandler<TemplateMessage, WebsocketSender> = {
    "SetText": (payload: SetTextPayload) => {
        setTextOnElement(payload.id, payload.text);
    },
    "AddClass": (payload: ManipulateClassPayload) => {
        addClassOnElement(payload.id, payload.class);
    },
    "RemoveClass": (payload: ManipulateClassPayload) => {
        removeClassOnElement(payload.id, payload.class);
    },
    "LoadAnimations": (payload: LoadAnimationsPayload) => {
        const state = getZagreusState();
        state.animationsSequences = payload.animations;
    },
    "LoadElements": (payload: LoadElementsPayload) => {
        const state = getZagreusState();
        state.elementConfigs = payload.elements;
        saveInitialAlignmentStates(state.elementConfigs);
        flattenUseElements(state.elementConfigs);
        flattenTransforms(state.elementConfigs);
    },
    "ExecuteAnimation": (payload: ExecuteAnimationPayload) => {
        applyAnimation(payload.animationSequence);
    },
    "OnLoad": (payload: OnLoadPayload) => {
        payload.animationSequences.forEach(sequence => applyAnimation(sequence));
        const maxTimeout = getMaxTimeoutFromSequences(payload.animationSequences);
        setTimeout(() => showZagreusSvgContainer(), maxTimeout);
    },
    "SetImageSource": (payload: SetImageSourcePayload) => {
        setImageSource(payload.id, payload.asset);
    },
    "LogError": () => {
    },
};

export class WebsocketHandler {
    constructor(private sender: WebsocketSender) {
    }

    handleMessage(message: string): void {
        console.log(message);
        const parsedMessage: TaggedEnumType<TemplateMessage> = JSON.parse(message);
        templateMessageHandlers[parsedMessage.tag](parsedMessage.payload, this.sender);
    }
}
