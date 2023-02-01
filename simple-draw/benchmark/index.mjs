import fs from "node:fs/promises";

// to make this work, have to edit package.json there to `"type": "module"`
// i dunno why this isn't the default...
import { load_font, DrawImage } from "../pkg/simple_draw.js";

import satori from "satori";
import { Resvg } from "@resvg/resvg-js";

const font = await fs.readFile("../tests/Inter-Medium.otf");
load_font("inter", font);
const sprite = await fs.readFile("../tests/sprite.png");

function makeImgDrawWasm() {
  const img = DrawImage.new(64, 64);
  img.fill_rectangle([0, 0, 32, 64], [255, 0, 128, 255]);
  img.fill_rectangle([32, 0, 64, 64], [0, 128, 255, 255]);
  img.draw_sprite_png([4, 4], sprite);
  img.draw_sprite_png([6, 6], sprite);
  const config = { rgba: [0, 255, 0, 255], size: 18, font_name: "inter" };
  img.draw_text([2, 26], config, "fk!");
  img.draw_text([28, 40], { ...config, rgba: [255, 0, 0, 64] }, "shiet");
  return img.get_png_image();
}

async function makeImgSatori() {
  const svg = await satori(
    {
      type: "div",
      props: {
        children: [
          {
            type: "span",
            children: "fk!",
          },
          {
            type: "span",
            children: "shiet",
          },
          {
            type: "img",
            props: {
              src: `data:image/png;base64,${sprite.toString("base64")}`,
            },
          },
          {
            type: "img",
            props: {
              src: `data:image/png;base64,${sprite.toString("base64")}`,
            },
          },
        ],
        style: { color: "#ff0088", fontFamily: "inter", display: "flex", },
      },
    },
    {
      width: 64,
      height: 64,
      fonts: [
        {
          name: "inter",
          data: font,
          weight: 500,
          style: "normal",
        },
      ],
    },
  );
  const opts = {
    background: "#fff",
    fitTo: {
      mode: "width",
      value: 64,
    },
  };
  const resvg = new Resvg(svg, opts);
  const pngData = resvg.render();
  const pngBuffer = pngData.asPng();
  return [pngData, pngBuffer];
}

async function bench(fn, iterations = 100) {
  const t0 = Date.now();
  for (let i = 0; i < iterations; i++) {
    await fn();
  }
  console.log(`${fn.name} done in ${Date.now() - t0} ms`);
}

await bench(makeImgDrawWasm);
await bench(makeImgSatori);
