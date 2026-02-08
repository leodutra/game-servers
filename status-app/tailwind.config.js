/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      backgroundImage: {
        'banana-tile': "url('images/bg-tile-3.jpg')",
      },
      colors: {
        banana: {
          bg: '#F5E6D3',
          card: '#FCF5E2',
          graph: '#F3E5D3',
          input: '#46556440',
        },
        brown: {
          DEFAULT: '#583017',
          dark: '#3E2723',
          light: '#8D6E63',
        },
        yellow: {
          DEFAULT: '#FFD633',
          light: '#FFF5CC',
          'btn-from': '#FFEB3B',
          'btn-to': '#FBC02D',
          'btn-hover-from': '#FFEB80',
          'btn-hover-to': '#FFD633',
        },
        green: {
          DEFAULT: '#5DBE5D',
          light: '#C3E6CB',
          dark: '#1B5E20',
          border: '#388E3C',
          to: '#4CAF50',
        },
        red: {
          DEFAULT: '#EF5350',
          light: '#F5C6CB',
          dark: '#7F0000',
          border: '#B71C1C',
          to: '#D32F2F',
        },
        hytale: '#5A6A7A',
      },
      fontFamily: {
        fredoka: ['Fredoka', 'sans-serif'],
      },
      dropShadow: {
        'brown-xs': '0 1px 0 #583017',
        'brown-sm': '0 2px 0 #583017',
        'brown-md': '0 2.5px 0 #583017',
        'brown-lg': '0 3px 0 #583017',
        'brown-xl': '0 4px 0 #583017',
        'green-xl': '0 6px 0 #1B5E20',
      },
      boxShadow: {
        'card': '0 6px 16px rgba(0,0,0,0.25)',
        'btn': '0 4px 8px rgba(0,0,0,0.2), inset 0 2px 0 rgba(255,255,255,0.5), inset 0 -4px 0 rgba(0,0,0,0.15)',
        'bevel': 'inset 6px 6px 10px rgba(88,48,23,0.2), inset -6px -6px 10px rgba(255,255,255,0.7)',
      }
    }
  },
  plugins: [
    function({ addUtilities, matchUtilities, theme }) {
      addUtilities({
        '.paint-stroke': { 'paint-order': 'stroke fill' },
      })
      matchUtilities(
        { 'text-stroke': (value) => ({ '-webkit-text-stroke': value }) },
        { values: {
          'main': '8px #583017',
          'main-osm': '4px #583017', // Optimized for mobile
          'main-outer': '16px rgba(255,255,255,0.9)',
          'main-outer-osm': '8px rgba(255,255,255,0.9)', // Optimized for mobile
          'server': '4.5px #583017',
          'stat': '5px #583017',
          'sm': '3px #583017', // labels, inputs
          'md': '4px #583017', // buttons, medium text
          'green-sm': '3px #1B5E20',
          'red-sm': '3px #7F0000',
          'uptime': '10px #1B5E20',
        }}
      )
    }
  ],
}
