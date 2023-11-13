//
//  network.h
//
//  Created by Emmanuel Mera on 08/11/2023.
//

#ifndef network_h
#define network_h

/*
 The idea is clearly to abstract all of this so the connect function
 MUST launch a new thread which can manage the connection indepedently of the execution of the game
 I want this behaviour so that, for future developements, it will be easer to make the protocol evolve
 (for example if we want to handle some packets indepentently from the game etc...)
 */

/*
 Initiate the connexion to the server
 */
extern void NETconnect(void);

/*
 Disconnect from the server (for a proper way to disconnect from the server)
 NOT URGENT (we can deal without it)
 */
extern void NETdisconnect(void);

/*
 Non-blocking function which send a packet to the server
 We want basically to add the packet to a list of packet the thread is loocking
 and sending
 */
extern void NETsend(void);

/*
 Non-blocking function which return the content of a packet if one is buffed
 */
extern void NETrecv(void);

/*
 Non-blocking function which ask the server to create a game
 */
extern void NETcreateGame(void);

/*
 Non-blocking function which ask the server to join a game
 */
extern void NETjoinGame(void);

/*
 Get status : CONNECTED ; DISCONNECTED ; IN-GAME ; IN-RUNNING-GAME
 */
extern void NETgetStatus(void);

/*
 Non-blocking function which ask the server to launch the game (go from IN-GAME to IN-RUNNING-GAME status)
 */
extern void NETstartGame(void);

#endif /* network_h */
