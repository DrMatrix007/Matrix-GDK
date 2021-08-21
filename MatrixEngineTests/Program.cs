﻿using System;
using MatrixEngine;
using MatrixEngine.Content;
using MatrixEngine.GameObjects;
using MatrixEngine.GameObjects.Components;
using MatrixEngine.GameObjects.Components.PhysicsComponents;
using MatrixEngine.GameObjects.Components.RenderComponents;
using MatrixEngine.GameObjects.Components.StateManagementComponents;
using MatrixEngine.GameObjects.Components.TilemapComponents;
using MatrixEngine.StateManagment;
using MatrixEngine.System;
using MatrixEngine.UI;
using SFML.Graphics;
using SFML.System;
using SFML.Window;

namespace MatrixEngineTests {
    class FPSProvider : Provider<float> {
        private App app;

        public void SetApp(App app) {
            this.app = app;
        }

        private new float data { get; set; }

        public override float Get() {
            return 1 / app.deltaTime;
        }
    }

    internal static class Program {
        private static void Main(string[] args) {
            var prov = new ComponentProvider<TilemapComponent>();

            var playerProv = new ComponentProvider<SimplePlayerControllerComponent>();

            var counterProv = new CounterProvider();

            const bool isDebug = false;

            var fpsProvider = new FPSProvider();


            App app = new App("Tests",
                isDebug,
                new Scene(
                    new GameObject[]
                    {
                        new GameObject(
                            new Vector2f(-10, -10),
                            new Component[]
                            {
                                new SpriteRendererComponent("Image1.png", 18, 2),
                                new ConsumerComponent<TilemapComponent>(prov),
                                new ProviderTesterComponent(),
                                new ConsumerComponent<int>(counterProv),
                                new SimplePlayerControllerComponent(),
                                new RigidBodyComponent(new Vector2f(0, 50f), new Vector2f(0.9f, 0.9f), false),
                                new ColliderComponent(ColliderComponent.ColliderType.Rect),
                                new CameraControllerComponent(),
                                new TestResizeTilemapComponent(prov)
                            }
                        ),
                        new GameObject(
                            new Component[]
                            {
                                new TilemapComponent(),
                                new TilemapRendererComponent(),
                                new ComponentProviderSetterComponent<TilemapComponent>(prov),
                                new RigidBodyComponent(true),
                                new ColliderComponent(ColliderComponent.ColliderType.Tilemap),
                            }
                        ),
                        new GameObject(
                            new Component[]
                            {
                                new SpriteRendererComponent("Image2.png", 400, 55),
                                new RigidBodyComponent(true),
                                new ColliderComponent(ColliderComponent.ColliderType.Rect),
                            }
                        )
                    },
                    new UIObject[]
                    {
                        new TextRendererConsumerUIObject(new Anchor(new Vector2f(), new Vector2f(20, 10)),
                            new ProviderConverter<string, float>(fpsProvider, e => e.ToString("00000.0")),
                            new UITextStyle(10, Color.White, Color.Cyan, FontManager.CascadiaCode,
                                isResize: true),
                            10),
                        new SpriteRendererUIObject(new Anchor(new Vector2f(0, 10), new Vector2f(10, 10)),
                            new Texture("Image1.png"), new UIStyle(1, Color.White, Color.Blue), 1)
                    }
                )
            );

            fpsProvider.SetApp(app);

            app.Run();
        }
    }

    public class ProviderTesterComponent : Component {

        public ProviderTesterComponent() {
        }

        public override void Start() {
            // app.asyncOperationManager.AddAsyncOperation(new AsyncOperation(Enumerator()));

            var p1 = GetComponent<ConsumerComponent<int>>();
            app.AddToDebug(p1.provider);

            var p = GetComponent<ConsumerComponent<TilemapComponent>>().GetOutput();
            if (p == null) {
                // yield break;
            }

            var r = new Random();
            // var t = new Texture("grass.png");
            if (p != null) {
                p.transform.scale = new Vector2f(1f, 1f);
                for (var i = 0; i < 1000; i++) {
                    for (var j = 0; j < 1000; j++) {
                        if (r.NextDouble() < 0.2)
                            p.SetTile(new Vector2i(i, j), new Tile(TextureManager.GetTexture("grass.png")));
                    }

                    if (i == 50) {
                        transform.position = new Vector2f(i, -50);
                    }
                }
            }
        }


        public override void Update() {
            var p = GetComponent<ConsumerComponent<int>>().provider as CounterProvider;

            if (app.keyHandler.isPressed(Keyboard.Key.G)) {
                p?.Add();
            }
        }
    }
}