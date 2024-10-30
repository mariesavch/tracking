
module.exports = {
    mode: "all",
    content: [
        // include all rust, html and css files in the src directory
        "./src/**/*.{rs,html,css}",
        // include all html files in the output (dist) directory
        "./dist/**/*.html",
    ],
  theme: {
    extend: {
      colors: {
        red: 'rgb(var(--red))',
        pink: 'rgb(var(--pink))',
        mauve: 'rgb(var(--pink))',
        peach: 'rgb(var(--peach))',
        yellow: 'rgb(var(--yellow))',
        green: 'rgb(var(--green))',
        teal: 'rgb(var(--teal))',
        blue: 'rgb(var(--blue))',
        text: 'rgb(var(--text))',
        subtext1: 'rgb(var(--subtext1))',
        subtext0: 'rgb(var(--subtext0))',
        overlay2: 'rgb(var(--overlay2))',
        overlay1: 'rgb(var(--overlay1))',
        overlay0: 'rgb(var(--overlay0))',
        surface2: 'rgb(var(--surface2))',
        surface1: 'rgb(var(--surface1))',
        surface0: 'rgb(var(--surface0))',
        base: 'rgb(var(--base))',
        mantle: 'rgb(var(--mantle))',
        crust: 'rgb(var(--crust))',
      },
      fontFamily: {
        sans: "Cartograph CF",
        mono: "Cartograph CF",
      },
    },
  },
  future: {
    hoverOnlyWhenSupported: true,
  },
}
