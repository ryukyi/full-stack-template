# Use an official nginx unprivileged image
FROM nginxinc/nginx-unprivileged:alpine

# Copy static files to nginx public folder
COPY ./index.html ./script.js /usr/share/nginx/html/

# Copy the custom Nginx configuration file into the container
COPY ./config/custom_nginx.conf /etc/nginx/conf.d/default.conf

# Expose port 8080
EXPOSE 8080