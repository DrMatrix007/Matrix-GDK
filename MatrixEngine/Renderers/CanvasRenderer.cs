﻿using MatrixEngine.UI;
using SFML.Graphics;
using SFML.System;
using System.Collections.Generic;
using System.Linq;
using MatrixEngine.System;

namespace MatrixEngine.Renderers {
    public class CanvasRenderer {

        RenderTexture target;


        App app;

        List<UIObject> list;

        public CanvasRenderer(App app) {
            this.app = app;
            list = new List<UIObject>();

            target = new RenderTexture((uint)app.windowSize.X, (uint)app.windowSize.Y);


        }

        public void Add(UIObject component) {
            list.Add(component);
        }

        public void Render() {
            if (target.Size != app.window.Size) {
                target.Dispose();
                target = new RenderTexture(app.window.Size.X, app.window.Size.Y);
            }
            target.Clear(Color.Transparent);
            ;

            var new_list = list.OrderBy(x => {
                return x.layer;
            });
            foreach (var component in new_list) {

                component.Render(target);


            }
            target.Display();

            var tmp = app.window.GetView();
            var window_size = app.window.Size;
            // ReSharper disable once PossibleLossOfFraction
            app.window.SetView(new View(new Vector2f(window_size.X / 2, window_size.Y / 2), (Vector2f)app.window.Size));
            /* draw your stuff */
            var sp = new Sprite(target.Texture);

            //sp.Position =
            //-(Vector2f)app.window.Size / 2;



            app.window.Draw(sp);

            app.window.SetView(tmp);

            list.Clear();

        }


    }
}