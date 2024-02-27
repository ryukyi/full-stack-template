# Use an official nginx unprivileged image
FROM nginxinc/nginx-unprivileged:alpine

# Copy static files to nginx public folder
COPY . /usr/share/nginx/html

# Expose port 80
EXPOSE 80