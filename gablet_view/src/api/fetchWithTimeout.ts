const fetchWithTimeout = async (resource: RequestInfo | URL, options: RequestInit & { timeout?: number } = {}) => {
  if (typeof resource === 'string') {
    resource = encodeURI(resource);
  }

  console.log("Fetching", resource);

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