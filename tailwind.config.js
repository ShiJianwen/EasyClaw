/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{vue,js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: "#6366F1",
          light: "#8B5CF6",
          lighter: "#A78BFA",
        },
        dark: {
          900: "#0F0F1A",
          800: "#1A1A2E",
          700: "#16213E",
        },
        text: {
          primary: "#F8FAFC",
          secondary: "#94A3B8",
          muted: "#64748B",
        },
        success: "#22C55E",
        danger: "#EF4444",
        warning: "#F59E0B",
        info: "#3B82F6",
      },
      fontFamily: {
        sans: [
          "PingFang SC",
          "-apple-system",
          "BlinkMacSystemFont",
          "Segoe UI",
          "Roboto",
          "sans-serif",
        ],
      },
      animation: {
        "glow-pulse": "glow-pulse 2s ease-in-out infinite",
        "shimmer": "shimmer 2s linear infinite",
        "float-up": "float-up 0.6s ease-out",
      },
      keyframes: {
        "glow-pulse": {
          "0%, 100%": { opacity: "0.6", filter: "blur(20px)" },
          "50%": { opacity: "1", filter: "blur(30px)" },
        },
        "shimmer": {
          "0%": { transform: "translateX(-100%)" },
          "100%": { transform: "translateX(100%)" },
        },
        "float-up": {
          "0%": { opacity: "0", transform: "translateY(10px)" },
          "100%": { opacity: "1", transform: "translateY(0)" },
        },
      },
    },
  },
  plugins: [require("tailwindcss-animate")],
};
