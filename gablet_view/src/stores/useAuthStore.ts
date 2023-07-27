import type { LoginResponse } from "@/api/auth";
import getApiSource from "@/api/getApiSource";
import { devLog } from "@/utils/errors";
import jwtDecode, { type JwtPayload } from "jwt-decode";
import { defineStore } from "pinia";

const useAuthStore = defineStore(
    'auth',
    {
        state: () => ({
            accessToken: '',
            refreshToken: '',
            loginExpires: 0,
            refreshExpires: 0,
            username: '',
            apiSource: getApiSource()
        }),
        getters: {
            isLoggedIn() : boolean {
                return Date.now() < this.loginExpires && Date.now() < this.refreshExpires;
            },
            canRefresh(): boolean {
                return Date.now() < this.refreshExpires;
            }
        },
        actions: {
            setLogin(loginResponse: LoginResponse): boolean {
                if (!loginResponse.access_token || !loginResponse.refresh_token) {
                    devLog("Invalid login response.");
                    return false;
                }

                try {
                    const access = jwtDecode<JwtPayload>(loginResponse.access_token);
                    this.loginExpires = (access.exp ?? 0) * 1000;
                    this.username = access.sub ?? '';
                    this.accessToken = loginResponse.access_token;
                    const refresh = jwtDecode<JwtPayload>(loginResponse.refresh_token);
                    this.refreshExpires = (refresh.exp ?? 0) * 1000;
                    this.refreshToken = loginResponse.refresh_token;
                } catch(err) {
                    devLog("Failed to decode token. Error: ", err);
                    return false;
                }

                return true;
            }
        },
        persist: true
    });

export type AuthStore = ReturnType<typeof useAuthStore>;

export default useAuthStore;