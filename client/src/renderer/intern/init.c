//
//  init.c
//
//  Created by Emmanuel Mera on 08/11/2023.
//

#include "raylib.h"

#include "init.h"

void RENinitWindow(void) {
    int screenWidth = 0, screenHeight = 0;
    
    InitWindow(screenWidth, screenHeight, "PhoneTile");
    
    SetTargetFPS(60);
}
