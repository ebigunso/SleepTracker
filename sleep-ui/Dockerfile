# SvelteKit dev container for sleep-ui
FROM node:20-alpine

WORKDIR /app

# Install deps first to leverage Docker layer cache (deterministic builds)
COPY package.json package-lock.json ./
RUN npm ci

# Copy rest of the app
COPY . .

EXPOSE 5173
# Run Vite dev server and listen on all interfaces for Docker port mapping
CMD ["npm", "run", "dev", "--", "--host", "0.0.0.0"]
