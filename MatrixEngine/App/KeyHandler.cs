﻿using SFML.Window;
using System;
using System.Collections.Generic;
using System.Linq;

namespace MatrixEngine {
    public sealed class KeyHandler {

        public enum KeyInput {
            Release,
            Press
        }

        private Dictionary<Keyboard.Key, bool> values;
        private Dictionary<Keyboard.Key, KeyInput> keysEvents;


        public KeyHandler() {

            values = new Dictionary<Keyboard.Key, bool>();
            keysEvents = new Dictionary<Keyboard.Key, KeyInput>();

            foreach (Keyboard.Key key in Enum.GetValues<Keyboard.Key>()) {
                try {

                    values[key] = false;

                } catch (Exception e) {
                    Debug.LogError(e.ToString());
                }
            }
        }
        private void SetKey(Keyboard.Key key, bool b) {
            try {
                values[key] = b;

            
            } catch (Exception e) { }
            pressedKeys = getCurrentPressedKeys();


        }
        public void PressedKey(Keyboard.Key key) {
            SetKey(key, true);
        }
        public void ReleasedKey(Keyboard.Key key) {
            SetKey(key, false);
        }

        public Keyboard.Key[] getCurrentPressedKeys() {
            return values.Where(
                    (e) => {
                        return e.Value;
                    }
                ).Select(
                    (e) => {
                        return e.Key;
                    }
            
                    ).ToArray();
        }

        public Keyboard.Key[] pressedKeys
        {
            private set;
            get;

        } = new Keyboard.Key[] { };

        

    }
}