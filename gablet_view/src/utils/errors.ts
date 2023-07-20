import type { SimpleTFunction } from "./strings";


export const getErrorMessage = (err: any, t: SimpleTFunction) : string => {
    let message = '';

    switch (typeof err) {
        case 'object':
            switch (typeof err.error) {
                case 'object':
                    message = tryParseErrorObject(err.error, t);
                    break;
                case 'string':
                    message = err.error;
                    break;
                default:
                    message = tryParseErrorObject(err, t);
                    break;
            }
            break;
        case 'string':
            message = err;
    }

    if (!message) {
        message = t('error.genericError');
    }

    return message;
}

const tryParseErrorObject = (err: any, t: SimpleTFunction) : string => {
    if (typeof err.toString === 'function') {
        return err.toString();
    }

    let builder = '';
    let lastWasErrorType = false;

    if (err.error_code) {
        builder += t('error.errorCode', { code: err.error_code });
    }

    if (err.error_message) {
        if (builder.length != 0) {
            builder += t('error.errorCodeSeparator');
        }

        builder += t('error.errorMessage', { message: err.error_message });
    }

    if (err.error_type) {
        if (builder.length != 0) {
            builder += t('error.errorMessageSeparator');
        }

        builder += t('error.errorType', { type: err.error_type });
        lastWasErrorType = true;
    }

    if (err.stack_trace) {
        if (builder.length != 0) {
            builder += t(lastWasErrorType ? 'error.errorTypeSeparator' : 'error.errorMessageSeparator');
        }
        
        builder += t('errorStackTrace', { stackTrace: err.stack_trace });
    }

    return builder;
}