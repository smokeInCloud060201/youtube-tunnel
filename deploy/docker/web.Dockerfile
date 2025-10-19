FROM node:20-alpine AS build

WORKDIR /app
COPY package*.json ./
RUN rm -f package-lock.json && npm cache clean --force
RUN npm install
COPY . .
RUN npm run build

FROM nginx:alpine

COPY nginx/default.conf /etc/nginx/conf.d/default.conf

COPY --from=build /app/dist /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
