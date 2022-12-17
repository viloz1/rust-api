FROM node:18.12.1-alpine as builder
WORKDIR /app
COPY . /app
RUN npm install -g @angular/cli
RUN npm install
RUN npm run build

FROM nginx:1.19.0
COPY --from=builder /app/dist/rust-api-frontend /usr/share/nginx/html
EXPOSE 4200
CMD ["nginx", "-g", "daemon off;"]