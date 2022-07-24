import camelcaseKeys from "camelcase-keys";

const BASE_URL = "http://localhost:8000";

export const constructUrl = (path: string) => new URL(path, BASE_URL);

const service = <T>(
  input: RequestInfo | URL,
  init?: RequestInit,
  json?: Record<string, any>
): Promise<T> =>
  fetch(input, {
    ...init,
    body: json ? JSON.stringify(json) : undefined,
    ...(init?.headers?.["Content-Type"] === "application/json"
      ? {
          headers: {
            "Content-Type": "application/json",
            ...init?.headers,
          },
        }
      : {}),
  })
    .then((res) => {
      if (res.status < 200 || res.status >= 300) {
        throw new Error(`${res.status} ${res.statusText}`);
      }
      return res.json();
    })
    .then((res) => camelcaseKeys(res, { deep: true }))
    .then((res) => res as T);

export interface CreatePasteResp {
  deleteToken: string;
  slug: string;
}

export const createPaste = async ({
  content,
}: {
  content: string;
}): Promise<CreatePasteResp> => {
  return service<CreatePasteResp>(
    constructUrl("/"),
    {
      method: "POST",
    },
    {
      content,
    }
  );
};

export const deletePaste = async ({
  slug,
  deleteToken,
}: {
  slug: string;
  deleteToken: string;
}): Promise<void> => {
  return service<void>(constructUrl(`/${slug}?token=${deleteToken}`), {
    method: "DELETE",
  });
};
