<script setup>
// import {active, isPermissionGranted, sendNotification} from '@tauri-apps/plugin-notification'
import * as THREE from 'three';
import {OrbitControls} from 'three/addons/controls/OrbitControls.js';
import {GLTFLoader} from 'three/addons/loaders/GLTFLoader.js';
import {BVHLoader, FBXLoader, OBJLoader} from "three/addons";
import WebGL from 'three/addons/capabilities/WebGL.js';
import Stats from 'three/addons/libs/stats.module.js';
import {nodeFrame} from 'three/addons/renderers/webgl-legacy/nodes/WebGLNodes.js';
import {onMounted, ref} from "vue";

import {open} from '@tauri-apps/plugin-dialog';
import {convertFileSrc} from '@tauri-apps/api/core';


const started = ref(false)
const filePath = ref("")
const fileName = ref("")
const _objLoader = new THREE.ObjectLoader();
const bvhLoader = new BVHLoader();
const fbxLoader = new FBXLoader();
const objLoader = new OBJLoader();
const gltfLoader = new GLTFLoader();


//GLOBAL VARS

const clock = new THREE.Clock();
let container, stats;

let camera, scene, renderer, controls, model, mixer;


const startRender = () => {
  const canvas = document.querySelector('#three')
  const face = document.getElementById('render-face')

  if (WebGL.isWebGLAvailable()) {

    // SCENE
    scene = new THREE.Scene();
    scene.background = new THREE.Color(0xa0a0a0);
    scene.fog = new THREE.Fog(0xa0a0a0, 15, 150);

    //CAMERA
    camera = new THREE.PerspectiveCamera(45, 3 / 2, 0.25, 200);
    camera.position.set(-5, 3, 10);

    // LIGHTS
    const hemiLight = new THREE.HemisphereLight(0xffffff, 0x8d8d8d, 3);
    hemiLight.position.set(0, 20, 0);
    scene.add(hemiLight);

    const dirLight = new THREE.DirectionalLight(0xffffff, 3);
    dirLight.position.set(0, 20, 10);
    // dirLight.castShadow = true;
    // dirLight.shadow.camera.top = 180;
    // dirLight.shadow.camera.bottom = -100;
    // dirLight.shadow.camera.left = -120;
    // dirLight.shadow.camera.right = 120;
    scene.add(dirLight);

    // SKYDOME
    // const topColor = new THREE.Color().copy(light.color);
    // const bottomColor = new THREE.Color(0xffffff);
    // const offset = 400;
    // const exponent = 0.6;
    //
    // const h = positionLocal.add(offset).normalize().y;
    //
    // const skyMat = new MeshBasicNodeMaterial();
    // skyMat.colorNode = vec4(mix(color(bottomColor), color(topColor), h.max(0.0).pow(exponent)), 1.0);
    // skyMat.side = THREE.BackSide;
    //
    // const sky = new THREE.Mesh(new THREE.SphereGeometry(4000, 32, 15), skyMat);
    // scene.add(sky);

    // RENDERER
    renderer = new THREE.WebGLRenderer({canvas, antialias: true});
    renderer.setPixelRatio(window.devicePixelRatio);
    renderer.shadowMap.enabled = true;
    const face_style = window.getComputedStyle(face)
    console.log(face_style.width, face_style.height)
    renderer.setSize(parseInt(face_style.width.slice(0, -2)), parseInt(face_style.height.slice(0, -2)),);

    // CONTROLS
    controls = new OrbitControls(camera, renderer.domElement);
    controls.maxPolarAngle = 0.9 * Math.PI / 2;
    controls.enableZoom = true;

    // STATS
    stats = new Stats({canvas: document.getElementById('stat')});
    const stat_dom = stats.dom
    stat_dom.style.position = 'absolute'
    face.appendChild(stat_dom);


    //Add CUBE
    // const geometry = new THREE.BoxGeometry(1, 1, 1);
    // const material = new THREE.MeshBasicMaterial({color: 0xc0c1c0});
    // const cube = new THREE.Mesh(geometry, material);
    // cube.position.setY(5)
    // scene.add(cube);

    // MESH AND GRID
    const mesh = new THREE.Mesh(new THREE.PlaneGeometry(200, 200), new THREE.MeshPhongMaterial({
      color: 0x595959,
      depthWrite: false
    }));
    mesh.rotation.x = -Math.PI / 2;
    scene.add(mesh);

    const grid = new THREE.GridHelper(200, 40, 0x000000, 0x000000);
    grid.material.opacity = 0.2;
    grid.material.transparent = true;
    scene.add(grid);

    let loader, obj_type = filePath.value.slice(-3)
    switch (obj_type) {
      case '':
        loader = '';
        break
      case 'glb':
        loader = gltfLoader;
        break
      case 'fbx':
        loader = fbxLoader;
        break
      case 'bvh':
        loader = bvhLoader;
        break
      case 'obj':
        loader = objLoader;
        break
    }
    // fileName.value += obj_type + loader
    if (loader) {
      loader.load(
          // 'lightmap.json',
          // 'RobotExpressive.glb',
          filePath.value,
          obj => {
            if (obj_type === 'glb') {
              model = obj.scene
              scene.add(model)
            } else if (obj_type === 'bvh') {
              const skeletonHelper = new THREE.SkeletonHelper(obj.skeleton.bones[0]);
              skeletonHelper.rotateX(-Math.PI / 2)
              obj.skeleton.bones[0].rotateX(-Math.PI / 2)

              scene.add(obj.skeleton.bones[0]);
              scene.add(skeletonHelper);

              // play animation
              mixer = new THREE.AnimationMixer(obj.skeleton.bones[0]);
              mixer.clipAction(obj.clip).play();
            } else if (obj_type === 'fbx') {
              mixer = new THREE.AnimationMixer(obj);
              const action = mixer.clipAction(obj.animations[0]);
              action.play();
              obj.traverse(function (child) {
                if (child.isMesh) {
                  child.castShadow = true;
                  child.receiveShadow = true;
                }
              });
              // obj.rotateX(-Math.PI / 2)
              model = obj
              scene.add(obj)
            } else {
              model = obj
              scene.add(model)
            }
            // console.log(model)
            if (model) {
              adjustSize()
              fitOnScreen()
            }
            // fileName.value += model
          },
          xhr => {
            console.log((xhr.loaded / xhr.total * 100) + '% loaded');
          }
          ,
          err => {
            // fileName.value += err + err.value + JSON.stringify(err) + err.message
          }
      )
    }

    // camera.position.z = 5;
    // console.log(model)
    // if (model) {
    //   console.log(getObjSize(model))
    //   fitOnScreen()
    // }

    function animate() {


      if (started) {
        // cube.rotation.x += 0.01;
        // cube.rotation.y += 0.01;
        requestAnimationFrame(animate)


        //渲染动画
        const delta = clock.getDelta();
        if (mixer) mixer.update(delta);

        nodeFrame.update();

        renderer.render(scene, camera)

        stats.update()

      }
    }

    animate()
  } else {
    const warning = WebGL.getWebGLErrorMessage();
    canvas.appendChild(warning);
  }

}


const filePicker = async () => {
  // Open a dialog
  const file = await open({
    multiple: false,
    directory: false,
    filters: [
      {
        name: '3D Obj',
        extensions: ['obj', 'fbx', 'glb', 'bvh'],
      },
    ],
  });
  if (file) {
    filePath.value = convertFileSrc(file.path)
    fileName.value = file.name
    return true
  }
  return false

// Prints file path and name to the console
}

function getObjSize(mesh) {
//mesh:模型
  const box = new THREE.Box3().setFromObject(mesh);
  return box.getSize(new THREE.Vector3())

}

function adjustSize(targetSize = 15) {
  const box = new THREE.Box3().setFromObject(model);
  const center = box.getCenter(new THREE.Vector3())
  const size = box.getSize(new THREE.Vector3())

  const x = (box.max.x - box.min.x)
  const y = (box.max.y - box.min.y)
  const z = (box.max.z - box.min.z)
  const maxDim = Math.max(x, y, z)
  const scale = targetSize / maxDim
  model.scale.set(scale, scale, scale)


  // const aspect = targetSize ? new THREE.Vector3().copy(targetSize.divide(size)) : new THREE.Vector3(1, 1, 1)
  // // 通过位置需要根据缩放比例进行调整
  // model.position.copy(position.sub(center).multiply(aspect))
  // model.scale.copy(aspect)
  // console.log(model)
}

function fitOnScreen() {
  const box = new THREE.Box3().setFromObject(model);
  console.log(box)
  const boxSize = box.getSize(new THREE.Vector3()).length();
  const boxCenter = box.getCenter(new THREE.Vector3());

  frameArea(boxSize * 1.25, boxSize, boxCenter);

  controls.maxDistance = boxSize * 10;
  controls.target.copy(boxCenter);
  controls.update();
}

function frameArea(sizeToFitOnScreen, boxSize, boxCenter) {
  const halfSizeToFitOnScreen = sizeToFitOnScreen * 0.5;
  const halfFovY = THREE.MathUtils.degToRad(camera.fov * 0.5);
  const distance = halfSizeToFitOnScreen / Math.tan(halfFovY);

  const direction = new THREE.Vector3()
      .subVectors(camera.position, boxCenter)
      .multiply(new THREE.Vector3(1, 0, 1))
      .normalize();

  camera.position.copy(direction.multiplyScalar(distance).add(boxCenter));

  camera.near = boxSize / 100;
  camera.far = boxSize * 100;
  camera.updateProjectionMatrix();
  camera.lookAt(boxCenter.x, boxCenter.y, boxCenter.z);
}

onMounted(() => {
  startRender();
  started.value = true
})

</script>

<template>
  <div class="main-container">
    <h2 style="text-align: center">三维手语教学平台</h2>
    <div id="render-face">
      <canvas id="three"></canvas>
      <!--      <canvas id="stat"></canvas>-->
    </div>
    <div class="row" style="padding-top: 2vh">
      <el-button @click="startRender();started=true">Load！</el-button>
      <!--    <el-button v-show="started" @click="startRender();started=false">Stop！</el-button>-->
      <el-button @click="filePicker().then(res=>{res?startRender():void(0)})">Select</el-button>
    </div>
    <el-text>{{ fileName }}</el-text>

  </div>
</template>

<style scoped>

#three {
  height: 600px;
  width: 900px;
}

#render-face {
  position: relative;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  text-align: center;
  border-radius: 4px;
  border: #393a3c 2px solid;
  width: 80vw;

}

.main-container {
  margin: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
  align-items: center;
}

canvas {
  border-radius: 4px;
}
</style>