import { getZagreusElement } from "../utils";

export const setTextOnElement = (elementName: string, text: string): void => {
  const element = getZagreusElement(elementName);
  element.innerText = text;
  return;
};
