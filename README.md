# PhoneTile

Welcome to PhoneTile !

Why playing together on it's own phone when you can play together on all the phones ?
PhoneTile is a small app for Android plaftforms that implement games that relies on the fact that players share their screens to play together !

# Basic usage

To use this app, you'll need several things :
- A bunch of firends (4-6 is a good amount)
- Phones that runs on android and in developer mode
- A computer with some way to custumize your firewall and a known local ip address
- And a bit of love

## Server

The app relies on a server so you'll need to use a computer and let the incomming traffic on the 8888 port go through.
Look [here](https://github.com/Phone-Tile/PhoneTile/actions/workflows/server.yml/badge.svg?branch=main) for details on how to build and run the server.

## Client

This part is the more fun ! You'll need to follow the 'not that complicated but long process" described [here](https://github.com/Phone-Tile/PhoneTile/actions/workflows/client.yml/badge.svg?branch=main).

## Usage

Now that you've build every thing, you just need to launch the server on your computer, download the apk on every phone and your set-and-ready to play !

# Contribute

The basic idea of the app is that it is possible and quite easy to add your own game to the show. We developped our framework so that our networking library do all of the linking stuff for you and you can completly focus on developping the app of your dreams ! The only small detail : you have to code in Rust. Every contribution or bug repport is relly apreaciated ! Just do a pull request or right a bug ticket and we'll be happy to discuss with you.

# Notes

For now, the software in very UNSTABLE. Please wait for the first stable release to have a reliable framework.
