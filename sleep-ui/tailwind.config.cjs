/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    container: {
      center: true,
      padding: {
        DEFAULT: '1.5rem',
        sm: '2rem',
        lg: '3rem'
      },
      screens: {
        sm: '640px',
        md: '768px',
        lg: '1024px',
        xl: '1280px',
        '2xl': '1440px'
      }
    },
    extend: {
      screens: {
        xs: '480px'
      },
      colors: {
        brand: {
          50: '#eef2ff',
          100: '#e0e7ff',
          200: '#c7d2fe',
          300: '#a5b4fc',
          400: '#818cf8',
          500: '#6366f1',
          600: '#4f46e5',
          700: '#4338ca',
          800: '#3730a3',
          900: '#312e81'
        }
      },
      spacing: {
        18: '4.5rem',
        22: '5.5rem',
        26: '6.5rem'
      },
      borderRadius: {
        xl: 'var(--radius-base)',
        lg: 'calc(var(--radius-base) - 0.15rem)',
        md: 'calc(var(--radius-base) - 0.3rem)'
      },
      boxShadow: {
        card: 'var(--shadow-card)'
      }
    }
  },
  plugins: [require('@tailwindcss/forms')]
};
