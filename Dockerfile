# Base image for building Rust application
FROM rust:alpine as builder
# Install build dependencies
RUN apk add --no-cache musl-dev
# Copy Rust project files
COPY matchmaker /matchmaker/
WORKDIR /matchmaker
# Build the Rust application in release mode in a single build layer
RUN cargo build --release

# Create the final image with Node.js and Python
FROM node:20-alpine
# Install Python and necessary dependencies
RUN apk add --no-cache python3 py3-pip python3-dev gcc musl-dev

# Set working directory

# Setup Python virtual environment
ENV VIRTUAL_ENV=/app/venv

RUN python3 -m venv $VIRTUAL_ENV
ENV PATH="$VIRTUAL_ENV/bin:$PATH"

# Copy the compiled binary from the builder stage

# Copy Node.js application files
COPY discord-bot/ /fellabot/discord-bot
COPY survey_collection /fellabot/survey_collection
COPY data/sample_input.csv /fellabot/data/sample_input.csv
COPY --from=builder /matchmaker/target/release/matchmaker /fellabot/matchmaker/target/release/matchmaker.exe

WORKDIR /fellabot/discord-bot

# Install Python requirements in the virtual environment
RUN pip install --no-cache-dir --upgrade pip && \
    pip install --no-cache-dir -r /fellabot/survey_collection/requirements.txt


# Install Node.js dependencies
RUN npm install --only=production

# Set environment variables if needed
ENV NODE_ENV=production
# Expose any necessary ports
# EXPOSE 3000

# Command to run the Node.js application
CMD ["npm", "run", "register-and-start"]