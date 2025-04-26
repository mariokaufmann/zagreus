export interface TaggedEnumType<T extends string, P = unknown> {
  tag: T;
  payload?: P;
}

export type EnumTypeHandler<T extends string, D> = {
  // eslint-disable-next-line
  [name in T]: (payload: any, data: D) => void;
};

export type ServerMessage =
  | "SetText"
  | "AddClass"
  | "RemoveClass"
  | "ExecuteAnimation"
  | "SetImageSource"
  | "SetCustomVariable"
  | "SetState";

export type ClientMessage = "LogError" | "StateSet";

export type AssetSource = "template" | "zagreus";
export type SetTextPayload = { id: string; text: string };
export type SetStatePayload = { name: string; value?: string };
export type ManipulateClassPayload = { id: string; class: string };
export type ExecuteAnimationPayload = {
  animationSequence: string;
  queueId: string | undefined;
};
export type SetImageSourcePayload = {
  id: string;
  asset: string;
  assetSource: AssetSource;
};
export type SetCustomVariablePayload = {
  name: string;
  value: string;
};
export type LogErrorPayload = { message: string; stack: string };
export type StateSetPayload = { name: string; value?: string };

export interface AnimationSequence {
  name: string;
  steps: AnimationStep[];
  onLoad?: boolean;
}

export interface AnimationStep {
  start: number;
  duration: number;
  animations: Animation[];
}

export interface Animation {
  id: string;
  name: string;
  direction: AnimationDirection;
  iterations: AnimationIterationCount;
}

export type AnimationDirection =
  | "normal"
  | "reverse"
  | "alternate"
  | "alternate-reverse";

export type AnimationIterationCount = "infinite" | number;
