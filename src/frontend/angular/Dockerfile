FROM node:latest as build-step
LABEL Author="Titanium IT 2023"

RUN mkdir -p /app
COPY package.json /app
WORKDIR /app
RUN npm install
COPY . /app
RUN npm run build --prod

FROM nginx:stable-alpine3.17
EXPOSE 80
COPY --from=build-step /app/dist/graphql-example-ui /usr/share/nginx/html
COPY nginx.conf  /etc/nginx/conf.d/default.conf