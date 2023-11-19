package com.phone_tile.game;
public class NativeLoader extends android.app.NativeActivity {
    static {
        System.loadLibrary("phone_tile");
    }
}
