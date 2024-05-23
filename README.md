# CryptoFlow

<p align="center">
  <img src="logo.png" height=50 width=50 />
</p>

`CryptoFlow` is a full-stack web application built with [Axum][0] and [SvelteKit][1]. It's a Q&A system tailored towards the world of cryptocurrency!

I also have the application live. You can interact with it [here][3]. Please note that the backend was deployed on [Render][4] which:

> Spins down a Free web service that goes 15 minutes without receiving inbound traffic. Render spins the service back up whenever it next receives a request to process. Spinning up a service takes up to a minute, which causes a noticeable delay for incoming requests until the service is back up and running. For example, a browser page load will hang temporarily.

Its building process is explained in [this series][2] of articles.

[0]: https://github.com/tokio-rs/axum "Ergonomic and modular web framework built with Tokio, Tower, and Hyper"
[1]: https://kit.svelte.dev/ "web development, streamlined"
[2]: https://dev.to/sirneij/series/25950 "CryptoFlow: Building a secure and scalable system with Axum and SvelteKit Series"
[3]: https://cryptoflow-one.vercel.app/ "CryptoFlow live application"
[4]: https://docs.render.com/free "Deploy for Free"
