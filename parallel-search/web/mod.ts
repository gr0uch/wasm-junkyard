import { serve } from "https://deno.land/std@0.140.0/http/server.ts";

const mimeMap = new Map([
  ["html", "text/html"],
  ["css", "text/css"],
  ["mjs", "application/javascript"],
  ["js", "application/javascript"],
  ["json", "application/json"],
  ["wasm", "application/wasm"],
]);

const pathMap = new Map([
  ["/", "/web/index.html"],
]);

const additionalHeaders = {
  "Cross-Origin-Embedder-Policy": "require-corp",
  "Cross-Origin-Opener-Policy": "same-origin",
};

const memCache = new Map();
const CACHE_TIME = 5 * 1000;

async function handleRequest(request: Request): Promise<Response> {
  const { pathname } = new URL(request.url);
  const mappedPath = pathMap.get(pathname) ?? pathname;
  const ext = mappedPath.match(/\.([^.]*?)$/)?.[1] ?? "";

  try {
    let file = memCache.get(mappedPath);

    if (!file) {
      file = await Deno.readFile(`./${mappedPath}`);
      memCache.set(mappedPath, file);
      setTimeout(() => {
        memCache.delete(mappedPath);
      }, CACHE_TIME);
    }

    return new Response(file, {
      headers: {
        ...additionalHeaders,
        "content-type": mimeMap.get(ext) ?? "application/octet-stream",
      },
    });
  } catch (e) {
    return new Response(e, {
      status: 404,
    });
  }
}

serve(handleRequest);
