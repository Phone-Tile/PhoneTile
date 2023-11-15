//
//  renderer.h
//
//  Created by Emmanuel Mera on 08/11/2023.
//

#ifndef renderer_h
#define renderer_h

// DEPENDENCY : object
#include "object.h"

/*
 I wrote a little bit of code here because I wanted to make sure my ideas were working,
 you can delete everything a rewrite it from scratch if you prefer I don't care (especially
 because I almost didn't wrote anythhing)
 */

/*
 The role of this module is to render the track, cars and visual effects
 It also handle general window oriented setups
 
 It is not done here but if the developper feal like
 it would be easier to break down RENrenderRaceFrame
 and to add functions like RENsetTrack, RENaddCar, etc
 free to do so !
 */

/*
 Initiate the window (fps, size and collect basic info on the device collected in a structure)
 */
extern void RENinitWindow(void);

/*
 Render the whole frame (depends on the objects module, which implements cars, tracks ...)
 */
extern void RENrenderRaceFrame(void);

/*
 Put it here for my own testing but whatever, this must go at some point, just until the game module is not cleaned up
 I prefer to keep it here
 */
extern void RENrenderTracks(OBJtrack *t);

#endif /* renderer_h */
