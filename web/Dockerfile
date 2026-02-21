# Build stage
FROM node:22-slim AS builder
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
RUN npm run build

# Run stage
FROM node:22-slim
WORKDIR /app
COPY --from=builder /app/.output ./
EXPOSE 3001
ENV PORT=3001
CMD ["node", "server/index.mjs"]
