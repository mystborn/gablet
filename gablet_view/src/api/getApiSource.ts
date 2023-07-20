import { stringHash } from "@/utils/strings";

const getApiSource = () => {
    return stringHash(navigator.userAgent);
}

export default getApiSource;