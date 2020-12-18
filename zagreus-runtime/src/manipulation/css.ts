export const ZagreusSvgContainerId: string = 'zagreus-svg-container';
export const ZagreusHiddenClass: string = 'zagreus-hidden';

export const addClassOnElement = (elementName: string, clazz: string) => {
    const element = document.getElementById(elementName);
    if (element) {
        element.classList.add(clazz);
    }
}

export const removeClassOnElement = (elementName: string, clazz: string) => {
    const element = document.getElementById(elementName);
    if (element) {
        element.classList.remove(clazz);
    }
}

export const showZagreusSvgContainer = () => {
    removeClassOnElement(ZagreusSvgContainerId, ZagreusHiddenClass);
}

