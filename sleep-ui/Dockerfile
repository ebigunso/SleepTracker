# SvelteKit dev container for sleep-ui
FROM node:20-alpine

WORKDIR /app

# Install deps first to leverage Docker layer cache
COPY package.json ./
# If a lockfile exists in the future, copy it too
# COPY package-lock.json ./
RUN npm install

# Copy rest of the app
COPY . .

EXPOSE 5173
# Run Vite dev server and listen on all interfaces for Docker port mapping
CMD ["npm", "run", "dev", "--", "--host", "0.0.0.0"]
