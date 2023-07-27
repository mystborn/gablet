import type { AuthStore } from "@/stores/useAuthStore";
import useAuthStore from "@/stores/useAuthStore";
import { devLog } from "@/utils/errors";

const fetchWithTimeout = async (
    resource: RequestInfo | URL,
    auth: boolean | AuthStore = false,
    options: RequestInit & { timeout?: number } = {}) => {
    if (typeof resource === 'string') {
        resource = encodeURI(resource);
    }

    if (auth) {
        if (typeof auth === 'boolean') {
            auth = useAuthStore();
        }

        if (!options.headers) {
            options.headers = new Headers({
                "Authorization": `Bearer ${auth.accessToken}`
            });
        } else if (options.headers instanceof Headers) {
            options.headers.set('Authorization', `Bearer ${auth.accessToken}`);
        } else {
            options.headers = new Headers({
                ...options.headers,
                'Authorization': `Bearer ${auth.accessToken}`
            });
        }
    }

    devLog("Fetching", resource);

    const { timeout = 10000 } = options;

    const controller = new AbortController();
    const id = setTimeout(() => controller.abort(), timeout);

    const response = await fetch(resource, {
        ...options,
        signal: controller.signal
    });
    clearTimeout(id);

    return response;
}

export default fetchWithTimeout;