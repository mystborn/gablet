import type { ErrorResult } from "./error";
import fetchWithTimeout from "./fetchWithTimeout";
import getApiSource from "./getApiSource";

const auth_server = import.meta.env.VITE_AUTH_SERVER;
const api_source = getApiSource();

export type RegisterRequest = {
    username: string,
    email: string,
    password: string
}

export type RegisterResponse = {
    access_token?: string,
    refresh_token?: string,
    error?: ErrorResult
}

const register = async ({ username, email, password }: RegisterRequest) : Promise<RegisterResponse> => {
    let response = await fetchWithTimeout(`${auth_server}/web/register?username=${username}&email=${email}&password=${password}&source=${api_source}`);

    let body = await response.json();

    if (response.ok) {
        return body as RegisterResponse;
    } else {
        return {
            error: body as ErrorResult
        }
    }
}

export type ValidateAccountRequest = {
    token: string,
    username: string
}

export type ValidateAccountResponse = {
    success: boolean,
    message?: string,
    error?: ErrorResult
}

const validate_account = async ({ token, username }: ValidateAccountRequest) : Promise<ValidateAccountResponse> => {
    let response = await fetchWithTimeout(`${auth_server}/api/validate?token=${token}&username=${username}`);
    let body = await response.json();

    if (response.ok) {
        return body as ValidateAccountResponse;
    } else {
        return {
            success: false,
            error: body as ErrorResult
        }
    }
}

export type LoginRequest = {
    username: string,
    password: string
}

export type LoginResponse = {
    access_token?: string,
    refresh_token?: string,
    error?: ErrorResult
}

const login = async ({ username, password }: LoginRequest) : Promise<LoginResponse> => {
    let response = await fetchWithTimeout(`${auth_server}/web/login?username=${username}&password=${password}`);
    let body = await response.json();

    if (response.ok) {
        return body as LoginResponse;
    } else {
        return {
            error: body as ErrorResult
        }
    }
}

export type RefreshResponse = {
    success: boolean,
    error?: ErrorResult
}

const refresh = async () : Promise<RefreshResponse> => {
    let response = await fetchWithTimeout(`${auth_server}/web/refresh?source=${api_source}`);

    if (response.ok) {
        return {
            success: true
        }
    } else {
        let body = await response.json() as ErrorResult;
        return {
            success: false,
            error: body
        }
    }
}

export default { register, validate_account, login, refresh };