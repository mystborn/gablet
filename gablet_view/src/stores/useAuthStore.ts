import type { LoginResponse } from "@/api/auth";
import getApiSource from "@/api/getApiSource";
import { devLog } from "@/utils/errors";
import jwtDecode, { type JwtPayload } from "jwt-decode";
import { defineStore } from "pinia";

const useAuthStore = defineStore(
    'auth',
    {
        state: () => ({
            loginExpires: 0,
            refreshExpires: 0,
            username: '',
            apiSource: getApiSource()
        }),
        getters: {
            isLoggedIn() : boolean {
                return Date.now() < this.loginExpires;
            },
            canRefresh(): boolean {
                return Date.now() < this.refreshExpires;
            }
        },
        actions: {
            setLogin(loginResponse: LoginResponse) {
                if (loginResponse.access_token) {
                    try {
                        const payload = jwtDecode<JwtPayload>(loginResponse.access_token);
                        this.loginExpires = (payload.exp ?? 0) * 1000;
                        this.username = payload.sub ?? '';
                    } catch(err) {
                        devLog("Failed to decode access token. Error: ", err);
                    }
                }

                if (loginResponse.refresh_token) {
                    try {
                        const payload = jwtDecode<JwtPayload>(loginResponse.refresh_token);
                        this.refreshExpires = (payload.exp ?? 0) * 1000;
                    } catch(err) {
                        devLog("Failed to decode refresh token. Error: ", err);
                    }
                }
            }
        },
        persist: true
    });

export type AuthStore = ReturnType<typeof useAuthStore>;

export default useAuthStore;