FROM node:alpine

# Install dependencies
COPY package*.json ./
RUN npm install

# Build frontend
COPY src ./src
COPY static ./static
COPY * ./
RUN npm run build

RUN npm ci --omit=dev

EXPOSE 3000
CMD ["node", "build"]
