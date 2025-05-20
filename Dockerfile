# Base image for building Rust application
FROM rust:alpine as builder
# Install build dependencies
RUN apk add --no-cache musl-dev
# Copy Rust project files - Note: this copies the *content* of the matchmaker dir
COPY matchmaker /matchmaker/
WORKDIR /matchmaker
# Build the Rust application in release mode in a single build layer
RUN cargo build --release

# Create the final image with Node.js and Python
FROM node:20-alpine
# Install Python and necessary dependencies
RUN apk add --no-cache python3 py3-pip python3-dev gcc musl-dev file

# Set working directory - let's set it to /app for consistency
WORKDIR /app

# Setup Python virtual environment
ENV VIRTUAL_ENV=/app/venv
RUN python3 -m venv $VIRTUAL_ENV
ENV PATH="$VIRTUAL_ENV/bin:$PATH"

# Copy application files
COPY discord-bot/ /app/discord-bot
COPY survey_collection /app/survey_collection
COPY data/sample_input.csv /app/data/sample_input.csv
# --- FIX: Copy num.txt from project root to /app/ ---
COPY matchmaker/num.txt /app/num.txt
# --- END FIX ---

# Copy the compiled binary from the builder stage
COPY --from=builder /matchmaker/target/release/matchmaker /app/matchmaker

# --- DEBUGGING STEPS (Optional, keep or remove) ---
# Verify if the files exist and check permissions
RUN ls -l /app/matchmaker /app/num.txt /app/data/sample_input.csv
# Check the file type of the binary
RUN file /app/matchmaker
# --- END DEBUGGING STEPS ---

# Ensure the Rust binary is executable
RUN chmod +x /app/matchmaker

# Install Python requirements in the virtual environment
RUN pip install --no-cache-dir --upgrade pip && \
    pip install --no-cache-dir -r /app/survey_collection/requirements.txt

# Change back to discord bot dir for npm install
WORKDIR /app/discord-bot

# Install Node.js dependencies
RUN npm install --only=production

# Set environment variables if needed
ENV NODE_ENV=production

# Expose any necessary ports
# EXPOSE 3000

# Command to run the Node.js application
CMD ["npm", "run", "register-and-start"]