import * as Comlink from "./comlink.mjs";

const worker = new Worker("/web/worker.mjs", { type: "module" });
const api = Comlink.wrap(worker);

await api.init();
const indexPtr = await api.SearchIndex();
await api.fetchResults(indexPtr, "/web/dictionary-compact.json");

const { container, input, list, info } = renderUI();
document.body.appendChild(container);

let workerSemaphore;
let waiterSemaphore;
let currentValue;
input.addEventListener("input", async (event) => {
  currentValue = event.target.value;

  if (workerSemaphore) {
    if (waiterSemaphore) return;
    waiterSemaphore = (async () => {
      await workerSemaphore;
      workerSemaphore = doSearch(currentValue);
      waiterSemaphore = null;
    })();
    return;
  }

  workerSemaphore = doSearch(currentValue);
  await workerSemaphore;
  workerSemaphore = null;
});

async function doSearch(value) {
  const t0 = Date.now();
  const len = list.children.length;
  const results = await api.searchSingleThread(indexPtr, value, len);
  info.textContent = `searched in ${Date.now() - t0} ms`;
  for (let i = 0; i < len; i++) {
    const html = results[i]?.[1] ?? "";
    list.children[i].innerHTML = html;
  }
}

function renderUI() {
  const container = document.createElement("div");
  const input = document.createElement("input");
  const info = document.createElement("span");
  const list = document.createElement("ol");

  input.autofocus = true;

  for (let i = 0; i < 10; i++) {
    list.appendChild(document.createElement("li"));
  }

  container.appendChild(input);
  container.appendChild(info);
  container.appendChild(list);

  return {
    container,
    input,
    info,
    list,
  };
}
