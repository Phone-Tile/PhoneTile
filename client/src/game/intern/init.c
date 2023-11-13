//
//  init.c
//
//  Created by Emmanuel Mera on 08/11/2023.
//

#include "init.h"

#include "init.h"
#include "raylib.h"
#include "MEM_alloc.h"

#include "ui.h"
#include "object.h"
#include "renderer.h"

/*
 THIS IS NOT CLEAN AT ALL, THIS SERVES ONLY AS AN EXAMPLE / TEST FILE FOR NOW
 ALL of this should be gone at some point
 */

///TODO: Clean it all up !

void callback(void) {
    DrawText("Hello you !", 120, 10, 20, WHITE);
}

void launch_main_screen(void) {
    int screenWidth = GetScreenWidth(), screenHeight = GetScreenHeight();
    
    /*
     Create event
     */
    Rectangle rec;
    rec.width = (float)screenWidth*3./5.;
    rec.height = (float)screenHeight/8.;
    rec.x = (float)screenWidth/5.;
    rec.y = 2.*(float)screenHeight/8.;
    
    /*
     Create button
     */
    UIwidget *w = UIcreateButtonWidget(rec, callback);
    UIaddWidget(w);
    
    /*
     Create track
     */
    OBJtrack *t = MEM_malloc(sizeof(OBJtrack), __func__);
    t->track_pieces_count = 1;
    t->track_pieces = MEM_calloc(sizeof(OBJtrackPiece), __func__);
    t->track_pieces->startPos.x = 0;
    t->track_pieces->startPos.y = GetScreenHeight()/5;
    t->track_pieces->endPos.x = GetScreenWidth();
    t->track_pieces->endPos.y = 4*GetScreenHeight()/5;
    t->track_pieces->startControlPos.x = GetScreenWidth()/2;
    t->track_pieces->startControlPos.y = GetScreenHeight()/5;
    t->track_pieces->endControlPos.x = GetScreenWidth()/2;
    t->track_pieces->endControlPos.y = 4*GetScreenHeight()/5;
    
    t->track_pieces->color = YELLOW;
    
    while (!WindowShouldClose()) {
        
        BeginDrawing();
        
        ClearBackground(BLACK);
        
        UIrenderWidgets();
        
        DrawText("START RACE", (int)(rec.x+(float)screenWidth/20.), (int)(rec.y+(float)screenWidth/20.), screenHeight/20, WHITE);
        
        UIupdateEvents();
        
        RENrenderTracks(t);
        
        EndDrawing();
    }
    
    CloseWindow();
}
