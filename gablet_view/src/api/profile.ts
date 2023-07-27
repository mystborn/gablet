import type { AuthStore } from "@/stores/useAuthStore";
import fetchWithTimeout from "./fetchWithTimeout";
import type { ErrorResult } from "./error";
import useAuthStore from "@/stores/useAuthStore";
import requestAsJson from "./requestAsJson";
import getApiSource from "./getApiSource";

const api_server = import.meta.env.VITE_API_SERVER;
const apiSource = getApiSource();

type ProfileResponse = {
    username?: string,
    error?: ErrorResult
}

const profile = async (): Promise<ProfileResponse> => {
    const auth = useAuthStore();
    let response = await fetchWithTimeout(`${api_server}/web/profile`, true, requestAsJson({
        username: auth.username
    }, apiSource));

    let body = await response.json();

    if (response.ok) {
        return body as ProfileResponse;
    } else {
        return {
            error: body as ErrorResult
        }
    }
}

export default { profile };