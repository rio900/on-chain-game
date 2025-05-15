<a id="readme-top"></a>

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/rio900/on-chain-game">
    <img src="static/images/logo.png" width="200" height="180" alt="Logo">
  </a>

  <h3 align="center">DotStriker!</h3>

  <p align="center">
    Even aliens need financial backing to invade space! Challenge your friends to join your orbit and race to collect the most coins in the galaxy. Get Striking! 🚀
    <br />
    <a href="https://github.com/rio900/on-chain-game/blob/main/README.md"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="#gameplay-demo">View Demo</a>
    &middot;
    <a href="#contributing">Contribute</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with-">Built With ♡</a></li>
        <li><a href="#why-polkadot">Why Polkadot?</a></li>
      </ul>
    </li>
    <li>
      <a href="#dotstriker-simulation-a-demo">DotStriker! Simulation: A Demo</a>
      <ul>
        <li><a href="#playing-dotstriker">Playing DotStriker!</a></li>
        <li><a href="#building-demo">Building DotStriker!</a></li>
      </ul>
    </li>
    <li><a href="#ui-design">UI Design</a></li>
      <ul>
        <li><a href="#screenshots">Screenshots</a></li>
        <li><a href="#wireframes">Wireframes</a></li>
      </ul>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#future-releases">Future Releases</a></li>
    <li>
      <a href="#meet-the-devs">Meet The Devs</a>
    </li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

 <img src="static/images/logo.png" width="800" height="300" alt="Gameplay">

DotStriker! is a real-time, multiplayer coin race — pilots control on-chain ships, collect coins across the Polkadot-powered arena, and battle for dominance.

No shooting. No luck. Just speed, skill, and sync.
The Striker who collects the most, wins.

#### 🎮 Features:

- 🚀 Real-time multiplayer combat in a cosmic arena
- 🪙 On-chain coin collection — every token counts toward your leaderboard rank
- 🧠 Skill-based gameplay with responsive touch controls and quick-strike mechanics
- 🛠 Wallet-native login with Polkadot identity
- 🌐 Decentralized backend — game logic and player state live directly on-chain

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With ♡

[![Rust][Rust]][Rust-url]
[![Unity][Unity]][Unity-url]
[![Polkadot][Polkadot]][Polkadot-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Why Polkadot?

> DotStriker! is a fully on-chain, real-time multiplayer space game built with Polkadot, the Polkadot SDK, and a Unity client.

🔧 Why Polkadot SDK (Pallet SDK)?

- 🧩 Runtime pallets (written in Rust) define the game’s core logic — like movement, scoring, and rewards — executed directly in the blockchain runtime.

- ⚡ Low-latency performance via WASM-based runtimes enables responsive real-time gameplay with deterministic, on-chain mechanics.

- 🔐 Polkadot’s shared security and interoperability allow DotStriker! to scale securely and connect to other parachains and ecosystems.

- 🔄 On-chain authority ensures game state and player actions are verifiable, tamper-proof, and transparent — no central game server needed.

🎮 Why Unity?

- 🎨 Unity powers a smooth, real-time visual experience while syncing with on-chain game state using custom networking bridges.

- 📱 Designed for eventual deployment across Web, Desktop, and Mobile platforms with wallet integration and Polkadot identity.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- DEMO -->

## DotStriker! Simulation: A Demo

### Playing DotStriker!

Watch a quick demo of how DotStriker works:

### Building DotStriker!

Watch our developers talk a little bit about the idea and how it was built:

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- DEMO -->

## UI Design

### Screenshots

### Wireframes

Like many ambitious projects, Dotstrikers! began as a humble proof-of-concept napkin sketch — here’s a glimpse at the game's earliest design ideas.

<img src="static/images/wf_registration.png" height=600 alt="Registration">
<img src="static/images/wf_gameplay.png" width="600" alt="Gameplay">

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->

## Getting Started

To get a local copy up and running, follow these simple example steps:

### Prerequisites

- npm
  ```sh
  npm install npm@latest -g
  ```

### Installation

1. Get a free API Key at [https://example.com](https://example.com)
2. Clone the repo
   ```sh
   git clone https://github.com/github_username/repo_name.git
   ```
3. Install NPM packages
   ```sh
   npm install
   ```
4. Enter your API in `config.js`
   ```js
   const API_KEY = "ENTER YOUR API";
   ```
5. Change git remote url to avoid accidental pushes to base project
   ```sh
   git remote set-url origin github_username/repo_name
   git remote -v # confirm the changes
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ROADMAP -->

## Roadmap

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- FUTURE RELEASES -->

## Future Releases

<ol>
  <li>Bigger maps, more players</li>
  <li>Collect custom themes and ship upgrades as NFTs</li>
  <li>Extend on all platforms, including desktop</li>
</ol>

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MEET THE DEVS -->

## Meet The Devs

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**. 🫶🏻

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement". Don't forget to give the project a star!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<!-- CONTACT -->

## Contact

Oyonika - [@oyonika](https://www.linkedin.com/in/oyonika) - oyonika@hotmail.com

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ACKNOWLEDGMENTS -->

## Acknowledgments

This project was made possible thanks to the support of some incredible platforms, collaborators and powerful open-source tools.

- [Rust](https://www.rust-lang.org/)
- [Unity](https://unity.com/solutions/programming)
- [Polkadot](https://polkadot.com/platform/sdk/)
- [EasyA](https://www.easya.io/)
- [Consensus 2025](https://consensus2025.coindesk.com/)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[Rust]: https://img.shields.io/badge/rust-000000?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/
[Unity]: https://img.shields.io/badge/unity-153225?style=for-the-badge&logo=solidity&logoColor=white
[Unity-url]: https://unity.com/games
[Polkadot]: https://img.shields.io/badge/polkadot-e5047a?style=for-the-badge&logo=solidity&logoColor=white
[Polkadot-url]: https://docs.polkadot.com/develop/networks/
