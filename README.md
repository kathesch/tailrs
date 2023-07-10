# tailrs

This is a command line tool for quickly initializing tailwind in a project directory. It is intended to lower the barrier to initializing tailwind in a barebone `index.html` only project directory and generally make it easier to experiment with simple web applciations.

### Installation

This tool requires **npm** and a global installation of tailwind's cli **tailwindcss**. 

```
npm install tailwindcss -g
```

### How it works

It simply creates a `tailwind.config.js` file and a `tailwind` directory with `input.css` and `output.css` files. `input.css` contains `@tailwind base; @tailwind components; @tailwind utilities;` as a default. `tailrs` will then run `npx tailwindcss -i tailwind/input.css -o tailwind/output.css --watch`. 

Any changes to `tailwind/input.css` will then be built on the fly by tailwind. 

