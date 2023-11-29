# PhoneTile

Welcome to PhoneTile !

Why play on separate screens when you can combine them all to live a unique game experience ?
PhoneTile is a small app for Android plaftforms that helps developer to implement games using a unique feature : the map is shared between the different screens and players can move everywhere !

For example, one can enjoy the [racer game](./server/src/game/racer/). Click on your phone and the car accelerates, switch phones and win the race against your friends !

## Basic usage

To use this app, you'll need :
- To like rust enough
- To write a front end and a back end file for your game (respectively in [the client side](./client/app/src/game/) and [the server side](./server/src/game/))
- And a bit of love and patience

## Server

The app relies on a server so you'll need to use a computer and let the incomming traffic on the 8888 port go through. </br>
Look [here](https://github.com/Phone-Tile/PhoneTile/blob/main/server/README.md) for details on how to build and run the server.

## Client

This part is the funniest ! You'll need to follow the "not that complicated but long process" described [here](https://github.com/Phone-Tile/PhoneTile/blob/main/client/README.md).

## Usage

Now that you've build every thing, you just need to launch the server on your computer, download the apk on every phone and you're set-and-ready to play !

# Contribute

The basic idea of the app is that it is possible and quite easy to add your own game to the show. We developped our framework so that our networking library do all of the linking stuff for you and you can completly focus on developping the app of your dreams ! The only small detail : you have to code in Rust. Every contribution or bug repport is relly apreaciated ! Just do a pull request or write a bug ticket and we'll be happy to discuss with you.

# Notes

For now, the software in very UNSTABLE. Please wait for the first stable release to have a reliable framework.
