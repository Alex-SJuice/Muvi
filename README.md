# Muvi

<img src=assets\unfold-muvi.png width=800 />

## Overview

It's going to capture system audio in real time (maybe even read what exact song is playing), and display a fancy, customizable visual in the desktop background. 
The hope is to have it not even show up on the taskbar.

Passive, Effortless, and Seamless (hopefully)


## Motivation

music visualizers are awesome, but it's annoying having them as separate tabs that clogs up the taskbar. Switching to it is also a pain

Having it as the desktop background would fix both issues! And also make the computer look sick

(I know this already exsists, but I wanted to make my own version so I can control exactly how it looks)


## Tech Used

It's 100% rust (for now)

loopback is using the cpal and wasapi libraries

The visualizer uses the fast fourier transform to analyze the music

I don't even know how I'm gonna write to the desktop background yet ;-;


## TODO LIST!?!?!?!?!? WOWOWOWOW

- Get system audio

	- Windows (cuz of course it's different)

	- everyone else

- analyze the audio in real time

- generate the pretty picture

- display it to the background
