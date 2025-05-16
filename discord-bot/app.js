import dotenv from "dotenv";
import path from 'node:path';
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

dotenv.config({ path: path.resolve(__dirname, "..", ".env") });

import express from 'express';
import fetch from 'node-fetch'; // make sure installed via npm
import {
  InteractionResponseType,
  InteractionType,
  verifyKeyMiddleware,
} from 'discord-interactions';
import { getRandomEmoji, getRandomInt } from './utils.js';
import { execFile, spawn } from 'child_process';
import fs from 'node:fs';
import os from 'node:os';


const app = express();
const PORT = process.env.PORT || 3000;


// Use express.raw() only on /interactions route for raw body buffer needed by discord-interactions
app.post(
  '/interactions',
  express.raw({ type: 'application/json' }),
  verifyKeyMiddleware(process.env.PUBLIC_KEY),
  async (req, res) => {
    // At this point, req.body is already parsed by verifyKeyMiddleware
    // So treat it as a normal object, no JSON.parse
    
    const payload = req.body;

    const { id, type, data, token, application_id } = payload;

    // ... your existing logic here (unchanged) ...
    
    // For example
    if(type === InteractionType.PING) {
      return res.send({ type: InteractionResponseType.PONG });
    }

    // Handle slash commands
    if (type === InteractionType.APPLICATION_COMMAND) {
      const { name } = data;

      if (name === 'roll') {
        let min;
        let max;
        
        if (!data.options) {
          min = 1;
          max = 100;
        } else {
          min = data.options[0].value;
          max = data.options[1].value;
        }

        const result = getRandomInt(min, max);
        return res.send({
          type: InteractionResponseType.CHANNEL_MESSAGE_WITH_SOURCE,
          data: { content: `Roll (${min}-${max}): ${result}` },
        });
      } 



      if (name === 'maketeams') {
        if (!data.options || data.options.length < 10) {
          console.log('maketeams: invalid options', data.options);
          return res.status(400).json({ error: 'You must specify exactly 10 players' });
        }

        // Set the helper file equal to how many solutions the user requested. Rust uses this later.

        if (data.options.length > 10) {
            fs.writeFileSync("../matchmaker/num.txt", String(data.options[10].value));
        }

        
        
        const users = data.options.slice(0, 10);
        const userIds = users.map((opt) => opt.value);
        
            
      
        // Defer the reply immediately
        res.json({ type: InteractionResponseType.DEFERRED_CHANNEL_MESSAGE_WITH_SOURCE });
      
        // Helper function to fetch username by userId
        async function fetchUsername(userId) {
          try {
            const response = await fetch(`https://discord.com/api/v10/users/${userId}`, {
              headers: {
                Authorization: `Bot ${process.env.DISCORD_TOKEN}`,
              },
            });
            if (!response.ok) {
              console.error(`Failed to fetch user ${userId}: ${response.status}`);
              return `UnknownUser(${userId})`;
            }
            const userData = await response.json();
            return userData.username;
          } catch (err) {
            console.error(`Error fetching user ${userId}:`, err);
            return `UnknownUser(${userId})`;
          }
        }
      
                // Fetch all usernames in parallel
        const usernames = await Promise.all(userIds.map(fetchUsername));

        const pythonProcess = spawn("python", ["../survey_collection/data.py", JSON.stringify(usernames)]);
        let pythonStderr = "";

        pythonProcess.stdout.on('data', (data) => {
          console.log(`Python STDOUT: ${data}`);
        });

        pythonProcess.stderr.on('data', (data) => {
          console.error(`Python STDERR: ${data}`);
          pythonStderr += data.toString(); // accumulate
        });

        pythonProcess.on('close', async (code) => {
          // Decide whether to proceed based on errors
          const match = pythonStderr.match(/ValueError:.*$/s);
          let content;
          let errorToDiscord = false;

          if (match) {
            // No need to replace any prefix, just use the full match
            content = match[0].slice(11);
            errorToDiscord = true;
          } else if (code !== 0) {
            content = "At least one invalid username";
            errorToDiscord = true;
          }

          if (errorToDiscord) {
            try {
              await fetch(
                `https://discord.com/api/v10/webhooks/${application_id}/${token}/messages/@original`,
                {
                  method: 'PATCH',
                  headers: { 'Content-Type': 'application/json' },
                  body: JSON.stringify({ content }),
                }
              );
            } catch (err) {
              console.error('Error notifying Discord about Python error:', err);
            }
            return; 
          }

          

          execFile(
            '../matchmaker/target/release/matchmaker.exe',
            {
              env: {
                ...process.env,
                USERS: userIds.join(','),
              },
            },
            async (error, stdout, stderr) => {
              if (error) {
                console.error('Rust error:', error);
                await fetch(
                  `https://discord.com/api/v10/webhooks/${application_id}/${token}/messages/@original`,
                  {
                    method: 'PATCH',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ content: 'Error running matchmaker.' }),
                  }
                );
                return;
              }

              // Compose message with usernames and Rust output
              const content = `Teams formed with players: ${usernames.join(', ')}\n\n${stdout.trim()}`;

              await fetch(
                `https://discord.com/api/v10/webhooks/${application_id}/${token}/messages/@original`,
                {
                  method: 'PATCH',
                  headers: { 'Content-Type': 'application/json' },
                  body: JSON.stringify({ content }),
                }
              );
            }
          );
        });
              
        return;
      }

      if (name === 'form') {
        return res.send({
          type: InteractionResponseType.CHANNEL_MESSAGE_WITH_SOURCE,
          data: { content: 'https://forms.gle/Ugy2zRhVFZ6jSgs89' },
          flags: 64
        });
      }

      if (name == "league") {
        return res.send({
          type: InteractionResponseType.CHANNEL_MESSAGE_WITH_SOURCE,
          data: { content: 'https://www.dotabuff.com/esports/leagues/18174-the-fellas-inhouse-league' },
          flags: 64
        });
      }

            if (name == "old") {
        return res.send({
          type: InteractionResponseType.CHANNEL_MESSAGE_WITH_SOURCE,
          data: { content: 'soumar old' },
          flags: 64
        });
      }

      console.error(`unknown command: ${name}`);
      return res.status(400).json({ error: 'unknown command' });
    }

    console.error('unknown interaction type', type);
    return res.status(400).json({ error: 'unknown interaction type' });
  }
);

app.listen(PORT, () => {
  console.log('Listening on port', PORT);
});