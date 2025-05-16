import dotenv from "dotenv";
import path from 'node:path';
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

dotenv.config({ path: path.resolve(__dirname, "..", ".env") });

import { InstallGlobalCommands } from './utils.js';

// Simple test command
export const ROLL = {
  name: 'roll',
  description: 'Basic command',
  type: 1, // Application Command Type: slash command
  options: [
    {name: "lower", description: "Lower Bound", type: 4, required: false},
    {name: "upper", description: "Upper Bound", type: 4, required: false},
  ]
};

// /maketeams command with 10 required user options
export const MAKE_TEAMS = {
  name: 'maketeams',
  description: 'Sorts 10 users into near-optimal inhouse teams. All users must have filled out /form.',
  type: 1,
  options: [
    { name: 'player1', description: 'Player 1', type: 6, required: true },
    { name: 'player2', description: 'Player 2', type: 6, required: true },
    { name: 'player3', description: 'Player 3', type: 6, required: true },
    { name: 'player4', description: 'Player 4', type: 6, required: true },
    { name: 'player5', description: 'Player 5', type: 6, required: true },
    { name: 'player6', description: 'Player 6', type: 6, required: true },
    { name: 'player7', description: 'Player 7', type: 6, required: true },
    { name: 'player8', description: 'Player 8', type: 6, required: true },
    { name: 'player9', description: 'Player 9', type: 6, required: true },
    { name: 'player10', description: 'Player 10', type: 6, required: true },
    {name: "num", description: "Top n Solutions", type: 4, required: false},
  ]
};

// /form command - returns a URL
export const GET_FORM = {
  name: 'form',
  description: 'Requests the URL for the inhouse questionnaire.',
  type: 1,
  options: [
    
  ]
};

export const GET_LEAGUE = {
  name: 'league',
  description: 'Requestsss the URL for the inhouse dotabuff page.',
  type: 1,
  options: [
    
  ]
};

export const GET_OLD = {
  name: 'old',
  description: 'old',
  type: 1,
  options: [
    
  ]
};



// All commands to register globally
const ALL_COMMANDS = [ROLL, MAKE_TEAMS, GET_FORM, GET_LEAGUE, GET_OLD];

// Register commands globally, called once or on startup
InstallGlobalCommands(process.env.APP_ID, ALL_COMMANDS);