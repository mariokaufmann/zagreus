export function getZagreusElement<T extends HTMLElement = HTMLElement>(
  name: string
): T {
  const element = document.querySelector(`[data-zag='${name}']`);
  if (!element) {
    throw new Error(
      `No element with data-zag attribute of value ${name} was found.`
    );
  }
  return element as T;
}
