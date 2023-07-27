import i18next from "i18next";
import I18NextVue from 'i18next-vue';
import LanguageDetector from "i18next-browser-languagedetector"
import type { App } from "vue";

const DEV = import.meta.env.DEV;

const en = {
    validate: {
        validateAccount: 'Validate Account',
        validating: 'Validating account',
        success: 'Successfully validated the account {{account}}',
        error: 'Failed to validate account: {{error}}',
        invalidUrl: 'Invalid URL'
    },
    signin: {
        usernameOrEmail: 'Username or Email',
        signIn: 'Sign In',
        register: 'Register',
        password: 'Password',
        username: 'Username',
        email: 'Email',
        submit: 'Submit',
        signInError: DEV ? 'Failed to sign in: {{error}}' : "Failed to sign in",
        registerError: DEV ? 'Failed to register: {{error}}' : "Failed to register",
        invalidResponse: 'Invalid server response'
    },
    error: {
        errorCode: 'Error {{code}}',
        errorCodeSeparator: ': ',
        errorMessage: '{{message}}',
        errorMessageSeparator: '\n',
        errorType: '{{type}}',
        errorTypeSeparator: ': ',
        errorStackTrace: '{{- stackTrace}}',
        genericError: 'Encountered an error'
    }
}

i18next
    .use(LanguageDetector)
    .init({
        debug: true,
        fallbackLng: 'en',
        resources: {
            en: {
                translation: en
            }
        }
    });

const useI18n = (app: App) : App => {
    return app.use(I18NextVue, { i18next });
}

export default useI18n;