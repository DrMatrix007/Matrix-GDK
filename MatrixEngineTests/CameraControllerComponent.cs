﻿
using MatrixEngine.GameObjects.Components;
using MatrixEngine.GameObjects.Components.PhysicsComponents;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;


namespace MatrixEngineTests {
    public class CameraControllerComponent : Component {
        public CameraControllerComponent() : base() {
            
        
        }
        public override void Start() {
        }

        public override void Update() {
            app.camera.position = position+transform.fullRect.size/2;
        }
    }
}
