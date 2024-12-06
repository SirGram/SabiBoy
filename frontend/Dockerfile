FROM node:20-slim
WORKDIR /app
COPY package*.json ./
RUN npm cache clean --force
RUN npm install --force
COPY . .
EXPOSE 5173
CMD ["npm", "run", "dev"]