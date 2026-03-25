# buongiorno-caffe-bot

![logo](./docs/images/readme-logo.png)

[![license-unlicense](https://img.shields.io/crates/l/buongiorno-caffe-bot.svg)](https://opensource.org/licenses/Unlicense)
[![repo-stars](https://img.shields.io/github/stars/veeso/buongiorno-caffe-bot?style=flat)](https://github.com/veeso/wasm-dbms/stargazers)
[![latest-version](https://img.shields.io/crates/v/buongiorno-caffe-bot.svg)](https://crates.io/crates/buongiorno-caffe-bot)
[![conventional-commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org)

[![ci](https://github.com/veeso/buongiorno-caffe-bot/actions/workflows/build.yml/badge.svg)](https://github.com/veeso/buongiorno-caffe-bot/actions)

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

The best Italian boomer flavored good morning and coffee moka related images at your command.

buongiorno-caffe-bot is a Telegram bot to receive the best italian boomer flavoured good morning and coffee moka related
images.

![Demo](./docs/images/demo.gif)

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

Scan this QR code or go to this URL <https://t.me/buongiornocaffe_bot> to start a chat with buongiorno-caffe-bot, then
add it to any group or chat directly with him.

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

## Powered by 💪

- [teloxide](https://github.com/teloxide/teloxide)
- [tokio](https://tokio.rs/)

---

## Changelog ⏳

View buongiorno-caffe-bot's changelog [HERE](CHANGELOG.md)

---

## License 📃

buongiorno-caffe-bot is licensed under the Unlicense license.

You can read the entire license [HERE](LICENSE)
