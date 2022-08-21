/**
 * Simple boids in canvas example using PixiJS
 * => https://pixijs.download/
 */
import { Universe, Rules } from "wasm-boids";
import * as PIXI from "pixi.js";

const rand = (max) => Math.floor(Math.random() * max);
const colorize = (color) => {
  const r = color > 10 ? "ff" : "aa";
  const g = color % 3 ? "ff" : "22";
  const b = color > 5 ? "ff" : "77";

  return `0x${r}${g}${b}`;
};

let width = window.innerWidth;
let height = window.innerHeight;
const count = 1000;

const app = new PIXI.Application({ resizeTo: window });
document.body.appendChild(app.view);

const boidGraphic = new PIXI.Graphics();
boidGraphic.beginFill(0xffffff);
boidGraphic.moveTo(-20, 10);
boidGraphic.lineTo(20, 0);
boidGraphic.lineTo(-20, -10);
const boidTexture = app.renderer.generateTexture(boidGraphic);

const universe = Universe.new(width, height);
const boidSprites = [];
for (let i = 0; i < count; i += 1) {
  const x = rand(width);
  const y = rand(height);
  const rotation = rand(360) * (Math.PI / 180);

  const rnd = i % 100 ? (i % 10 ? (i % 2 ? 3 : 6) : 8) : 15;
  const scale = (2 + rnd / 2) / 10;
  const vspeed = 2 / Math.sqrt(scale);
  const speed = vspeed;

  const rules = Rules.new(speed, scale, 100, 3, 0.1, 0.005, 0.2, 0.001);

  universe.create_boid(x, y, rotation, rules);

  const sprite = new PIXI.Sprite(boidTexture);
  sprite.tint = colorize(rnd);
  sprite.scale.x = scale;
  sprite.scale.y = scale;
  sprite.x = x;
  sprite.y = y;
  sprite.rotation = rotation;
  app.stage.addChild(sprite);
  boidSprites.push(sprite);
}

const drawBoids = () => {
  for (let i = 0; i < count; i += 1) {
    const boid = universe.get_boid(i);
    const boidSprite = boidSprites[i];

    boidSprite.x = boid.x();
    boidSprite.y = boid.y();
    boidSprite.rotation = boid.rotation();
  }
};

let mouseX, mouseY;
window.addEventListener("mousemove", (e) => {
  mouseX = e.clientX;
  mouseY = e.clientY;

  setTimeout(() => {
    mouseX = 0;
    mouseY = 0;
  }, 1000);
});

window.addEventListener("resize", () =>
  universe.resize(window.innerWidth, window.innerHeight, false)
);

const renderLoop = () => {
  universe.tick(mouseX, mouseY);
  drawBoids();
  requestAnimationFrame(renderLoop);
};
renderLoop();
