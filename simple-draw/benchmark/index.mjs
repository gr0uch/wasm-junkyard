import fs from "node:fs/promises";
import { loadFont, DrawImage } from "../pkg/simple_draw.js";

import satori from "satori";
import { Resvg } from "@resvg/resvg-js";

const font = await fs.readFile("../tests/Inter-Medium.otf");
loadFont("inter", font);
const sprite = await fs.readFile("../tests/sprite.png");

await fs.writeFile("output.png", makeImgSimpleDraw());

function makeImgSimpleDraw() {
  const img = DrawImage.new(64, 64);
  img.fillRectangle([0, 0, 32, 64], [255, 0, 128, 255]);
  img.fillRectangle([32, 0, 64, 64], [0, 128, 255, 255]);
  img.drawSpritePNG([4, 4], sprite);
  img.drawSpritePNG([6, 6], sprite, { resize: [32, 32], filter_type: "Lanczos3" });
  const config = { rgba: [0, 255, 0, 255], size: 18, font_name: "inter" };
  img.drawText([2, 26], config, "fk!");
  img.drawText([28, 40], { ...config, rgba: [255, 0, 0, 64] }, "shiet");
  return img.getPNGImage();
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

await bench(makeImgSimpleDraw);
await bench(makeImgSatori);
