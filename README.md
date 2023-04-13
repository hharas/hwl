# Hwl
Command-line Engine that transliterates Arabic text to English (Experimental)

![Hwl Command-line Demo GIF](demo.gif)

## Installation

You can either build everything yourself with cargo or walk in the easy route:

```bash
git clone https://codeberg.org/amad/hwl.git
cd hwl
./install.sh
```

## Why and how?

I couldn't find one single good website that actually transliterates Arabic text to English, or specifically [*"romanise"*](https://en.wikipedia.org/wiki/Romanization_of_Arabic) it. Non-Arabic speaking People can benefit from such tools to understand Arabic Pronunciation, whether from a religious (such as reciting the Noble Qur'an) or an educational manner.

I decided to develop Hwl, which I've unironically named a mistransliteration I saw of the word "حَوِّلْ" (*ḥawwil*) that means "Convert". Hwl isn't really a full-blown transliteration engine, yet at least, but rather a good start of a project helping Non-Arabic speakers pronounce Arabic.

Hwl is an Engine that replaces common arabic letter combinations with their appropriate pronunciation instead of what *Robots* make it seem to be pronounced. Hwl currently only supports Arabic text with Tashkeel, however, in the long run, we aim to make Hwl able to transliterate even Tashkeel-less Arabic.

## What is what?

#### `src/engine.rs`

The Hwl Engine module which contains the `transliterate()` public function that does all the work.

#### `src/main.rs`

The Hwl Command-line wrapper that makes use of `src/engine.rs`.

## TODOs

- [ ] Rewrite the Project in other Languages (Python, C and JavaScript).
- [ ] Add Support for more common combinations.
- [ ] Add a Web front-end.

## Contribution

Contributions to Hwl are welcomed. If you have anything worth adding to the project then please add it!

## License

Hwl is licensed under the GNU General Public License 3.0.
