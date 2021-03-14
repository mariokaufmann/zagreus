export const ZagreusSvgContainerId = 'zagreus-svg-container';
export const ZagreusHiddenClass = 'zagreus-hidden';

export const addClassOnElement = (elementName: string, clazz: string): void => {
    const element = document.getElementById(elementName);
    if (element) {
        element.classList.add(clazz);
    }
};

export const removeClassOnElement = (elementName: string, clazz: string): void => {
    const element = document.getElementById(elementName);
    if (element) {
        element.classList.remove(clazz);
    }
};

export const showZagreusSvgContainer = (): void => {
    removeClassOnElement(ZagreusSvgContainerId, ZagreusHiddenClass);
};

