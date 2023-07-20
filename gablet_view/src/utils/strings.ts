import type { TOptions } from "i18next";

export type SimpleTFunction = (key: string, options?: TOptions | string) => string;

export const stringHash = (str: string) : number => {
    let hash = 0;
    for (let i = 0, len = str.length; i < len; i++) {
        let chr = str.charCodeAt(i);
        hash = (hash << 5) - hash + chr;
        hash |= 0; // Convert to 32bit integer
    }
    return hash;
}
