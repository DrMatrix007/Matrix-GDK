﻿using SFML.System;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using MatrixEngine.Physics;

namespace MatrixEngine.GameObjects.Components {
    public class TransformComponent {

        private Vector2f _position = new Vector2f(0,0);

        public Vector2f position
        {
            set {
                _position = value;
                _rect.SetPos(value);
            }
            get => _position;
        }

        private Rect _rect = new Rect(0,0,0,0);

        public Rect fullRect
        {
            get => new Rect(_rect.X,_rect.Y,_rect.width*Scale.X,_rect.height*Scale.Y);
        }
        public Rect rect
        {
            get => _rect;
            set => _rect = value;
        }

        public Vector2f Scale = new Vector2f(1,1);




    }
}
