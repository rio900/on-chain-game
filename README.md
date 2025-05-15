<a id="readme-top"></a>

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/rio900/on-chain-game">
    <img src="static/images/logo.png" width="300" height="300" alt="Logo">
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
      <a href="#backstory">Dev Backstory</a>
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

This section should list any major frameworks/libraries used to bootstrap your project. Leave any add-ons/plugins for the acknowledgements section. Here are a few examples.

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

<img src="static/images/wf_registration.png"  alt="Registration">
<img src="static/images/wf_gameplay.png" width="800" height="300" alt="Gameplay">

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->


## 🚀 Getting Started with on-chain-game

### Prerequisites

This guide walks you through setting up and running the on-chain-game node locally for development and testing.

✅ Prerequisites

Before you begin, ensure you have the following installed:
* Rust (with rustup): Install from https://rustup.rs
* Substrate dependencies: Follow the official Substrate installation guide https://docs.substrate.io/install/
* Git: For cloning the repository


### Installation

📥 1. Clone the Repository
Clone the repository to your local machine:

git clone https://github.com/rio900/on-chain-game.git

<pre>
cd on-chain-game
</pre>
🛠️ 2. Build the Project
Build the project in release mode using Cargo:

<pre>
cargo build --release
</pre>

This command compiles the node with optimizations, resulting in a faster executable.

▶️ 3. Run the Node
After building, you can start a local development node with runtime debug logging enabled:

<pre>
RUST_LOG=runtime=debug ./target/release/solochain-template-node --dev --execution=wasm
</pre>

Explanation of the command:

* RUST_LOG=runtime=debug: Enables detailed logging for the runtime module, useful for debugging.
* ./target/release/solochain-template-node: Executes the compiled node binary.
*	--dev: Runs the node in development mode, using temporary keys and in-memory state.
*	--execution=wasm: Forces the node to execute using WebAssembly for runtime validation.

🧪 4. Interact with the Node
Once the node is running, you can interact with it using:
	•	Polkadot.js Apps: Connect via https://polkadot.js.org/apps and select the local node.
	•	Substrate Front-End Template: Clone and run https://github.com/substrate-developer-hub/substrate-front-end-template to interact with your node.

🛠️ 5. Customize and Develop
Explore the runtime modules in the runtime directory to understand and modify the game’s logic. You can add new pallets or modify existing ones to extend functionality.


<!-- ROADMAP -->

## Roadmap

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

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

Use this space to list resources you find helpful and would like to give credit to. I've included a few of my favorites to kick things off!

- [Choose an Open Source License](https://choosealicense.com)
- [GitHub Emoji Cheat Sheet](https://www.webpagefx.com/tools/emoji-cheat-sheet)
- [Malven's Flexbox Cheatsheet](https://flexbox.malven.co/)
- [Malven's Grid Cheatsheet](https://grid.malven.co/)
- [Img Shields](https://shields.io)
- [GitHub Pages](https://pages.github.com)
- [Font Awesome](https://fontawesome.com)
- [React Icons](https://react-icons.github.io/react-icons/search)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[Rust]: https://img.shields.io/badge/rust-000000?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/
[Unity]: https://img.shields.io/badge/unity-153225?style=for-the-badge&logo=solidity&logoColor=white
[Unity-url]: https://unity.com/games
[Polkadot]: https://img.shields.io/badge/polkadot-e5047a?style=for-the-badge&logo=solidity&logoColor=white
[Polkadot-url]: https://docs.polkadot.com/develop/networks/
