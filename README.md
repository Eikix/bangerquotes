## bangerquotes
### just another cli tool for generating quotes

<img width="557" alt="image" src="https://user-images.githubusercontent.com/66871571/191100283-6d596126-d373-4710-8c24-985510115b66.png">


#### How to setup?

##### Build the project and run the binary

- Install [rust](https://www.rust-lang.org/tools/install) if you haven't yet.

- Clone the repo. Cd into the repo.

- Run `cargo build --release`

- Go into your ~/.zshrc (or equivalent): run `code -r ~/.zshrc`.
And add to the alias list: 

```bash
# .zshrc

alias quote="~/<YOUR_PATH_TO_THE_REPO>/bangerquotes/target/release/bangerquotes"
```

- Restart your terminal

- Type `quote <honor | humor | proverb | philosophy | history | science | random>` from anywhere in your terminal. That's it, enjoy!

