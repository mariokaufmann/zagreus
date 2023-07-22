import { ZagreusContainerId } from "../constants";
import { getZagreusElement } from "../utils";

export const ZagreusHiddenClass = "zagreus-hidden";

export const addClassOnElement = (elementName: string, clazz: string): void => {
  const element = getZagreusElement(elementName);
  element.classList.add(clazz);
};

export const removeClassOnElement = (
  elementName: string,
  clazz: string,
): void => {
  const element = getZagreusElement(elementName);
  element.classList.remove(clazz);
};

export const showZagreusContainer = (): void => {
  removeClassOnElement(ZagreusContainerId, ZagreusHiddenClass);
};
