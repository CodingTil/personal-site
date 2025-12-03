---
slug: flappyking
image: <img loading="lazy" src="images/projects/FlappyKing/train.webp" alt="FlappyKing Training"/>
title: FlappyKing
color: bg-cyan-600
tagline: NEAT algorithm learning to play Flappy Bird
repository_url: https://github.com/CodingTil/FlappyKing
date_range: July 2019 - January 2020
skills: [java, lwjgl, git]
filters: [games, ai]
---
# Overview
After graduating from highschool, I set myself the goal to implement and train an AI from scratch. After working myself into the topic by implementing a Smart Rocket Genetic Algorithm project similar to [this one](https://thecodingtrain.com/CodingChallenges/029-smartrockets.html), I felt pretty confident about implementing and training a neural network.

Firstly, I implemented a copy of Flappy Bird with the [Lightweight Java Game Library](https://www.lwjgl.org/).

<img loading="lazy" src="images/projects/FlappyKing/train.webp" alt="FlappyKing Training"/>

After that however, I really struggeled implementing the AI. I tried a few approaches (Reinforcement Learning, Supervised Learning), but - even if I implemented them correctly - there was no training progress visible at all. So I kinda dropped the project and moved on.

A few months later however, I found [this website](https://xviniette.github.io/FlappyLearning/). While I was scanning its repository, I for the first time heard about Neuroevolution. After researching the topic a bit, I felt confident to implement my own NEAT algorithm using [the original paper](http://nn.cs.utexas.edu/downloads/papers/stanley.ec02.pdf) and other resources.

Today the project consists of various gamemodes such as:
- Single-player
- Co-op multiplayer
- Training
- Play vs. Computer

<img loading="lazy" src="images/projects/FlappyKing/menu.webp" alt="FlappyKing Menu"/>

# Implementation

## Game
As mentioned above, the game is implemented using the [Lightweight Java Game Library](https://www.lwjgl.org/).
For the purpose of this project however, it is extremly unnecessary and overkill. Java Swing would probably be the easiest way to go in Java.

## AI
[This article here](https://towardsdatascience.com/neat-an-awesome-approach-to-neuroevolution-3eca5cc7930f) is a great introduction into the NEAT algorithm.

The short version is: You combine neural networks with genetic algorithms, where the topoligy of the neural network is evolved from the most basic neural network into a *smart ai* over several generations.

The training process in my project is working on a different thread than the game is running in. To visualize the training process, the game always displays the current best neural network. This way, I can also speed up the training process quite a lot. Within 1-2 minutes of training the algorithm can produce great performing neural networks.