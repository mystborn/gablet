import useAuthStore, { type AuthStore } from "@/stores/useAuthStore"
import jwtDecode, { type JwtPayload } from "jwt-decode";
import { devLog, getErrorMessage } from "./errors";
import api from "@/api/api";
import { useTranslation } from "i18next-vue";
import type { SimpleTFunction } from "./strings";

export const isLoggedIn = async (auth: AuthStore, t: SimpleTFunction) : Promise<boolean> => {
    try {
        if (auth.isLoggedIn) {
            return true;
        }

        if (!auth.canRefresh) {
            return false;
        }

        const response = await api.auth.refresh({ refresh_token: auth.refreshToken });
        if (!response.error) {
            auth.setLogin(response);
            return true;
        }

        console.log(response);
        devLog("Failed to refresh login. Error: ", getErrorMessage(response.error, t));
        return false;
    } catch(err) {
        devLog(err);
        auth.$reset();
        return false;
    }
}