import type { ErrorResult } from "./error";
import fetchWithTimeout from "./fetchWithTimeout";
import getApiSource from "./getApiSource";
import requestAsJson from "./requestAsJson";

const authServer = import.meta.env.VITE_AUTH_SERVER;
const apiSource = getApiSource();

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

const register = async (request: RegisterRequest) : Promise<RegisterResponse> => {
    // let response = await fetchWithTimeout(`${auth_server}/web/register?username=${username}&email=${email}&password=${password}&source=${api_source}`);
    let response = await fetchWithTimeout(`${authServer}/api/register`, false, requestAsJson(request, apiSource));

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

const validate_account = async (request: ValidateAccountRequest) : Promise<ValidateAccountResponse> => {
    // let response = await fetchWithTimeout(`${auth_server}/api/validate?token=${token}&username=${username}`);
    let response = await fetchWithTimeout(`${authServer}/api/validate`, false, requestAsJson(request, apiSource));
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

const login = async (request: LoginRequest) : Promise<LoginResponse> => {
    let response = await fetchWithTimeout(`${authServer}/api/login`, false, requestAsJson(request, apiSource));
    let body = await response.json();

    if (response.ok) {
        return body as LoginResponse;
    } else {
        return {
            error: body as ErrorResult
        }
    }
}

export type RefreshRequest = {
    refresh_token: string
}

const refresh = async (request: RefreshRequest) : Promise<LoginResponse> => {
    let response = await fetchWithTimeout(`${authServer}/api/refresh`, false, requestAsJson(request, apiSource));
    let body = await response.json();

    if (response.ok) {
        return body as LoginResponse;
    } else {
        return {
            error: body as ErrorResult
        }
    }
}

export default { register, validate_account, login, refresh };