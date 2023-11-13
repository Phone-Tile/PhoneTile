//
//  object.h
//
//  Created by Emmanuel Mera on 08/11/2023.
//

#ifndef object_h
#define object_h

// DEPENDENCY : none

#include "raylib.h"

/*
 This file is here only to defined general objects type (track, cars ...) because they are conna be used by game and renderer
 */

typedef struct {
    Vector2 startPos;
    Vector2 endPos;
    Vector2 startControlPos;
    Vector2 endControlPos;
    Color   color;
} OBJtrackPiece;

typedef struct {
    int track_pieces_count;
    
    OBJtrackPiece *track_pieces;
} OBJtrack;

#endif /* object_h */
