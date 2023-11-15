//
//  physic.h
//
//  Created by Emmanuel Mera on 08/11/2023.
//

#ifndef physic_h
#define physic_h

// DEPENDENCY : none

/*
 This is here for now, it must be change to server directory and coded in rust, oopsi daisy
 */

extern void PHYsetTrack(void);
extern void PHYcreateCar(void);
extern void PHYcarAccelerate(void);
extern void PHYcomputeNextFrame(void);

#endif /* physic_h */
