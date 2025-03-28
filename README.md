# buongiorno-caffe-bot

<p align="center">
  <img src="/docs/images/buongiorno-caffe-bot.png" alt="logo" width="256" height="256" />
</p>

<p align="center">~ The best italian boomer flavoured good morning and coffee moka related images at your command ~</p>

<p align="center">Developed by <a href="https://veeso.me" target="_blank">@veeso</a></p>
<p align="center">Current version: 0.3.1 (28/03/2025)</p>

<p align="center">
  <a href="https://opensource.org/licenses/Unlicense"
    ><img
      src="https://img.shields.io/badge/License-Unlicense-teal.svg"
      alt="License-Unlicense"
  /></a>
  <a href="https://github.com/veeso/buongiorno-caffe-bot/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso/buongiorno-caffe-bot.svg"
      alt="Repo stars"
  /></a>
  <a href="https://crates.io/crates/buongiorno-caffe-bot"
    ><img
      src="https://img.shields.io/crates/d/buongiorno-caffe-bot.svg"
      alt="Downloads counter"
  /></a>
  <a href="https://crates.io/crates/buongiorno-caffe-bot"
    ><img
      src="https://img.shields.io/crates/v/buongiorno-caffe-bot.svg"
      alt="Latest version"
  /></a>
  <a href="https://ko-fi.com/veeso">
    <img
      src="https://img.shields.io/badge/donate-ko--fi-red"
      alt="Ko-fi"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/veeso/buongiorno-caffe-bot/actions/workflows/build.yml"
    ><img
      src="https://github.com/veeso/buongiorno-caffe-bot/workflows/Build/badge.svg"
      alt="Build CI"
  /></a>
</p>

---

- [buongiorno-caffe-bot](#buongiorno-caffe-bot)
  - [About buongiorno-caffe-bot 📰](#about-buongiorno-caffe-bot-)
  - [Command API 🐚](#command-api-)
  - [Get started 🏁](#get-started-)
    - [Users](#users)
    - [Developers](#developers)
  - [Roadmap 🎯](#roadmap-)
  - [Support the developer ☕](#support-the-developer-)
  - [Powered by 💪](#powered-by-)
  - [Contributing and issues 🤝🏻](#contributing-and-issues-)
  - [Changelog ⏳](#changelog-)
  - [License 📃](#license-)

---

## About buongiorno-caffe-bot 📰

buongiorno-caffe-bot is a Telegram bot to receive the best italian boomer flavoured good morning and coffee moka related images.

![Demo](/docs/images/demo.gif)

---

## Command API 🐚

- `/auguri`

    get a happy birthday image

- `/buongiornissimo`

    get a good morning image

- `/buonpomeriggio`

    get a good afternoon image

- `/buonanotte`

    get a good night image

- `/buonaserata`

    get a good evening image

- `/buonweekend`

    get a good weekend image

- `/buonpranzo`

    get a good lunch image

- `/buonacena`

    get a good dinner image

- `/compleanno <name> <date>`

    configure a birthday

- `/caffeee`

    This command will make the bot to send automatic messages

    > ❗ Automatic messages includes daily images

- `/puliziakontatti`

    Opt out from automatic messages

- `/help`

    display help

---

## Get started 🏁

### Users

Scan this QR code or go to this URL <https://t.me/buongiornocaffe_bot> to start a chat with buongiorno-caffe-bot, then add it to any group or chat directly with him.

![telegram-qr](/docs/images/qr-code-md.webp)

### Developers

If you want to develop on this bot, you can follow these simple steps:

1. Clone this repository `git clone git@github.com:veeso/buongiorno-caffe-bot.git`
2. Create your bot with the [Botfather](https://t.me/botfather)
3. Get your API key
4. Set your API key in your environment using the variable `TELOXIDE_TOKEN`
5. Set your database path in your environment using the variable `DATABASE_URI`
6. Touch the database file `touch $DATABASE_URI`
7. Run the buongiorno-caffe bot

---

## Roadmap 🎯

1. Add more celebrations (such as valentine, 25th april, 2nd june, ...)
2. Add more image providers such as <https://ticondivido.it> and <https://buongiornissimocaffe.it>

---

## Support the developer ☕

If you like buongiorno-caffe-bot and you're grateful for the work I've done, please consider a little donation 🥳

You can make a donation with one of these platforms:

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)

---

## Powered by 💪

- [teloxide](https://github.com/teloxide/teloxide)
- [tokio](https://tokio.rs/)

---

## Contributing and issues 🤝🏻

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve buongiorno-caffe-bot, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog ⏳

View buongiorno-caffe-bot's changelog [HERE](CHANGELOG.md)

---

## License 📃

buongiorno-caffe-bot is licensed under the Unlicense license.

You can read the entire license [HERE](LICENSE)
