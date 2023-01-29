import init, { SearchIndex, match_single } from "../pkg/multi_search.js";

(async () => {
  const [_, dictionary] = await Promise.all([
    init(),
    (async () => {
      const res = await fetch("dictionary-compact.json");
      return res.json();
    })(),
  ]);

  const index = SearchIndex.new();

  const t0 = Date.now();
  let i = 0;
  for (const word in dictionary) {
    index.load_result(word);
    i++;
  }
  console.log(`loaded ${i} results in ${Date.now() - t0} ms`);

  const { container, input, list, info } = renderUI();
  document.body.appendChild(container);

  let debounceTimer;
  input.addEventListener("input", (event) => {
    const { target: { value } } = event;
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(async () => {
      await Promise.resolve();
      doSearch(value);
    }, 250);
  });

  function doSearch(value) {
    const t0 = Date.now();
    const len = list.children.length;
    const results = index.search_single_thread(value, len);
    for (let i = 0; i < len; i++) {
      list.children[i].innerHTML = results[i]?.[1] ?? "";
    }
    info.textContent = `searched in ${Date.now() - t0} ms`;
  }
})();

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
