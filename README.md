# relevance-realization-rs

## Description

This is an implementation of the relevance realization theory of consciousness written fully in Rust. It is based on the work of Brett P. Andersen, Mark Miller and John Vervaeke. The theory is described in the following papers:

- [Predictive processing and relevance realization: exploring convergent solutions to the frame problem](https://link.springer.com/article/10.1007/s11097-022-09850-6)
- [Relevance Realization and the Emerging Framework in Cognitive Science](https://www.semanticscholar.org/paper/Relevance-Realization-and-the-Emerging-Framework-in-Vervaeke-Lillicrap/be78221d060225b0bd2d93e962e77a592d615473)

## Usage

This demonstration is a simple game with agents on a 2D map. Their goal is to maximise three parameters; saturation, thirst and health. By being on certain tiles, agents can increase or decrease these parameters. The agents are controlled through an economic system where they have to balance exploring and exploiting the map.

To move around the map use the WASD keys and Z/X to zoom. Clicking E will let you focus on an agent.
