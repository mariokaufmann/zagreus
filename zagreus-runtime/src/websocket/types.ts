export interface TaggedEnumType<T extends string, P = unknown> {
  tag: T;
  payload?: P;
}

export type EnumTypeHandler<T extends string, D> = {
  // eslint-disable-next-line
  [name in T]: (payload: any, data: D) => void;
};

export type TemplateMessage =
  | "SetText"
  | "LogError"
  | "AddClass"
  | "RemoveClass"
  | "ExecuteAnimation"
  | "SetImageSource";

export type AssetSource = "template" | "zagreus";
export type SetTextPayload = { id: string; text: string };
export type OnLoadPayload = { animationSequences: string[] };
export type ManipulateClassPayload = { id: string; class: string };
export type ExecuteAnimationPayload = { animationSequence: string };
export type SetImageSourcePayload = {
  id: string;
  asset: string;
  assetSource: AssetSource;
};
export type LogErrorPayload = { message: string; stack: string };

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
