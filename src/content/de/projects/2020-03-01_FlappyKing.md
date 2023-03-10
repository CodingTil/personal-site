---
slug: flappyking
image: <img loading="lazy" src="images/projects/FlappyKing/train.webp" alt="FlappyKing Training"/>
title: FlappyKing
color: bg-cyan-600
tagline: NEAT Algorithmus lernt Flappy Bird zu spielen
url: https://github.com/CodingTil/FlappyKing
date_range: Juli 2019 - Januar 2020
skills: [java, lwjgl, git]
filters: [games, ai]
---
# Übersicht
Nach dem Abitur setzte ich mir das Ziel, eine KI von Grund auf zu implementieren und zu trainieren. Nachdem ich mich in das Thema eingearbeitet hatte, indem ich ein Smart Rocket Genetic Algorithm-Projekt ähnlich wie [dieses](https://thecodingtrain.com/CodingChallenges/029-smartrockets.html) implementierte, fühlte ich mich ziemlich sicher, was die Implementierung und das Training eines neuronalen Netzwerks anging.

Als erstes habe ich eine Kopie von Flappy Bird mit der [Lightweight Java Game Library](https://www.lwjgl.org/) implementiert.

<img loading="lazy" src="images/projects/FlappyKing/train.webp" alt="FlappyKing Training"/>

Danach stieß ich jedoch auf Schwierigkeiten, als ich versuchte, die KI zu implementieren. Ich habe ein paar Ansätze ausprobiert (Reinforcement Learning, Supervised Learning), aber - selbst wenn ich sie richtig implementiert habe - war überhaupt kein Trainingsfortschritt sichtbar. Also habe ich das Projekt fallen gelassen und bin weitergezogen.

Ein paar Monate später fand ich jedoch [diese Website](https://xviniette.github.io/FlappyLearning/). Während ich das Repository überflog hörte ich zum ersten Mal von Neuroevolution. Nachdem ich das Thema ein wenig recherchiert hatte, fühlte ich mich zuversichtlich, meinen eigenen NEAT-Algorithmus mithilfe von [dem originalen paper](http://nn.cs.utexas.edu/downloads/papers/stanley.ec02.pdf) und anderen Ressourcen zu implementieren.

Heute besteht das Projekt aus verschiedenen Spielmodi wie:
- Einzelspieler
- Koop-Mehrspieler
- Training
- Spieler vs. Computer

<img loading="lazy" src="images/projects/FlappyKing/menu.webp" alt="FlappyKing Menu"/>

# Implementation

## Spiel
Wie oben erwähnt, ist das Spiel mit der [Lightweight Java Game Library](https://www.lwjgl.org/) implementiert.
Für den Zweck und Umfang dieses Projekts ist LWJGL jedoch extrem unnötig und ein Overkill. Java Swing wäre wahrscheinlich der einfachste Weg, um es in Java zu implementieren.

## KI
[Dieser Artikel hier](https://towardsdatascience.com/neat-an-awesome-approach-to-neuroevolution-3eca5cc7930f) ist eine gute Einführung in den NEAT-Algorithmus.

Die Kurzversion ist: Man kombiniert neuronale Netze mit genetischen Algorithmen, wobei die Topologie des neuronalen Netzes über mehrere Generationen vom einfachsten neuronalen Netz zu einer *intelligenten KI* weiterentwickelt wird.

Der Trainingsprozess in meinem Projekt läuft in einem anderen Thread als das Spiel. Um den Trainingsprozess zu visualisieren, wird im Spiel immer das aktuell beste neuronale Netz angezeigt. Auf diese Weise kann ich den Trainingsprozess auch sehr stark beschleunigen. Innerhalb von 1-2 Minuten Training kann der Algorithmus gut funktionierende neuronale Netze erzeugen.