const requestAsJson = (request: any, source?: number, method: string = 'POST'): RequestInit => {
    if (source) {
        request.source = `${source}`;
    }

    return {
        method,
        headers: new Headers({
            "Content-Type": "application/json"
        }),
        body: JSON.stringify(request)
    }
}

export default requestAsJson;