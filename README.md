# tailrs

This is a command line tool for quickly initializing tailwind in a project directory. It is intended to lower the barrier to initializing tailwind in a barebone web application directory. This should help set up small prototype web applications for learning tailwind.

### Installation

This tool requires **npm** and a global installation of tailwind's cli **tailwindcss**. 

```
npm install tailwindcss -g
```

tailrs can then be installed using brew. Presently, there is only a Arm64 MacOS release. 

```
brew tap kathesch/tailrs https://github.com/kathesch/tailrs
brew install tailrs
```

The command can then be run with `tailrs` in the terminal. 

### How it works

It simply creates a `tailwind.config.js` file and a `tailwind` directory with `input.css` and `output.css` files. `input.css` contains `@tailwind base; @tailwind components; @tailwind utilities;` as a default. `tailrs` will then run `npx tailwindcss -i tailwind/input.css -o tailwind/output.css --watch`. 

Any changes to `tailwind/input.css` will then be built on the fly by tailwind. 

