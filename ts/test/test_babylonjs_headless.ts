import {expect} from 'chai';
import * as BABYLON from 'babylonjs';

// Force loading loaders.
import * as BABYLON_LOADERS from 'babylonjs-loaders';
import GLTFLoaderCoordinateSystemMode = BABYLON_LOADERS.GLTFLoaderCoordinateSystemMode;
import VertexBuffer = BABYLON.VertexBuffer;
import Mesh = BABYLON.Mesh;
const gltfLoaderCoordinateSystemMode = GLTFLoaderCoordinateSystemMode;

describe('Test BabylonJS headless.', () => {

  it('Should test headless render.', (done) => {
    (global as any).XMLHttpRequest = require('xhr2').XMLHttpRequest;

    const engine = new BABYLON.NullEngine();
    const scene = new BABYLON.Scene(engine);

    const light = new BABYLON.PointLight("Omni", new BABYLON.Vector3(20, 20, 100), scene);

    const camera = new BABYLON.ArcRotateCamera("Camera", 0, 0.8, 100, BABYLON.Vector3.Zero(), scene);

    BABYLON.SceneLoader.ImportMesh("", "https://playground.babylonjs.com/scenes/", "skull.babylon", scene, function (newMeshes) {
      camera.target = newMeshes[0] as any;

      console.log("Meshes loaded from babylon file: " + newMeshes.length);
      for (let index = 0; index < newMeshes.length; index++) {
        console.log(newMeshes[index].toString());
      }

      BABYLON.SceneLoader.ImportMesh("", "https://www.babylonjs.com/Assets/DamagedHelmet/glTF/", "DamagedHelmet.gltf", scene, function (meshes) {
        console.log("Meshes loaded from gltf file: " + meshes.length);
        for (let index = 0; index < meshes.length; index++) {
          console.log(meshes[index].toString());
          console.log(typeof(meshes[index].getIndices()));
          /*const indices = meshes[index].getIndices();
          if (!indices) {
            throw new Error("No indices in mesh.");
          }
          console.log(typeof(indices));
          const positions = (meshes[index] as Mesh).getVerticesData(VertexBuffer.PositionKind, false, false);
          if (!positions) {
            throw new Error("No positions in mesh.");
          }
          console.log(typeof(positions));*/
        }

        console.log("render started")
        engine.runRenderLoop(function() {
          scene.render();
          engine.dispose();
          done();
        })
      });

    });

  }).timeout(20000);

});
