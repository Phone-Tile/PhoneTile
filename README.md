# PhoneTile
[![Server](https://github.com/Phone-Tile/PhoneTile/actions/workflows/server.yml/badge.svg?branch=main)](https://github.com/Phone-Tile/PhoneTile/actions/workflows/server.yml)
[![Client](https://github.com/Phone-Tile/PhoneTile/actions/workflows/client.yml/badge.svg?branch=main)](https://github.com/Phone-Tile/PhoneTile/actions/workflows/client.yml)

Welcome to PhoneTile !

Why play on separate screens when you can combine them all to live a unique gaming experience ?
PhoneTile is a small app for Android platforms where developers can implement their games using a unique feature : the game map is shared between any number of phone screens, and players can move wherever they want on the map !

For example, you can launch the game [RACER](./server/src/game/racer/). Press on your phone and the car accelerates, move your car from your screen to your friend's screen and win the race !

## How to create your game on PhoneTile

To code your game on this app, you'll need :
- To like rust enough
- To write a frontend and a backend file for your game (respectively in [the client side](./client/app/src/game/) and [the server side](./server/src/game/))
- And a bit of love and patience

### Server

The app relies on a server so you'll need to use a computer and let the incomming traffic on the 8888 port go through. </br>
Look [here](https://github.com/Phone-Tile/PhoneTile/blob/main/server/README.md) for details on how to build and run the server.

### Client

This part is the funnest ! You'll need to follow the not-that-complicated-but-long process described [here](https://github.com/Phone-Tile/PhoneTile/blob/main/client/README.md).

### Usage

Now that you've built everything, you just need to launch the server on your computer, download the apk on every phone and you're set-and-ready to play !

## Contribute

We've developed the app with modularity in mind : it is possible and quite easy to add your own game to PhoneTile. We developed our framework so that our networking library does all of the linking stuff for you and you can completely focus on developing the app of your dreams ! One tiny detail : you have to code in Rust. Every contribution or bug report is relly apreaciated ! Just do a pull request or write a bug ticket and we'll be happy to discuss it with you.

## Notes

For now, the software is very UNSTABLE. Please wait for the first stable release to have a reliable framework.
