//
//  track.c
//
//  Created by Emmanuel Mera on 08/11/2023.
//

#include "raylib.h"

#include "object.h"

#include "track.h"

/* ---------------------------- */
/*  local functions             */
/* ---------------------------- */

static void renderTrackPiece(OBJtrackPiece *p);

/* ---------------------------- */
/*  implementation              */
/* ---------------------------- */

void RENrenderTracks(OBJtrack *t) {
    OBJtrackPiece *p = t->track_pieces;
    
    for (int i=0; i<t->track_pieces_count; i++) {
        renderTrackPiece(p);
        
        p++;
    }
}

/* ---------------------------- */
/*  local functions             */
/* ---------------------------- */

void renderTrackPiece(OBJtrackPiece *p) {
    DrawLineBezierCubic(p->startPos, p->endPos, p->startControlPos, p->endControlPos, 5., p->color);
}
