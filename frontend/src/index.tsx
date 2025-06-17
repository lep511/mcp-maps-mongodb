/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
*/
import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { InMemoryTransport } from "@modelcontextprotocol/sdk/inMemory.js";
import { Transport } from "@modelcontextprotocol/sdk/shared/transport.js";
import { GoogleGenAI, mcpToTool } from '@google/genai';
import { ChatState, marked, Playground } from './services/playground';

import { startMcpGoogleMapServer } from './services/mcp_maps_server';

/* --------- */


async function startClient(transport: Transport) {
  const client = new Client({ name: "AI Studio", version: "1.0.0" });
  await client.connect(transport);
  return client;
}

/* ------------ */


const SYSTEM_INSTRUCTIONS = `You are an expert travel assistant specializing 
in discovering optimal short-term rental locations worldwide. 
You excel at using mapping tools and location data to identify accommodations 
that match specific criteria. When users request accommodation 
searches, use available tools. Always explain what are you doing.`;

const EXAMPLE_PROMPTS = [
  'Beachfront rentals with ocean views in Miami',
  'Family-friendly apartments near Central Park NYC',
  'Cozy cabins in the mountains of Colorado',
  'Modern lofts in downtown San Francisco',
  'Budget-friendly studios near UCLA campus',
  'Luxury penthouses with city views in Chicago',
  'Pet-friendly cottages in Portland Oregon',
  'Ski-in ski-out chalets in Aspen Colorado',
  'Waterfront condos in Seattle Washington',
  'Find a romantic getaway rental near Napa Valley',
  'Quiet retreats near Yellowstone National Park',
  'Downtown condos walking distance to Broadway NYC',
  'Lakeside cabins with private docks in Minnesota',
  'Arts district lofts in Detroit Michigan',
  'Beach houses with pools in Outer Banks NC',
  'Farm stays with horseback riding in Kentucky',
  'Rooftop terraces overlooking Golden Gate Bridge',
  'Historic Victorian homes in San Antonio Texas',
  'Eco-friendly treehouses in Olympic National Forest',
];

const ai = new GoogleGenAI({
  apiKey: process.env.API_KEY,
});

function createAiChat(mcpClient: Client) {
  return ai.chats.create({
    model: 'gemini-2.5-flash-preview-05-20',
    config: {
      systemInstruction: SYSTEM_INSTRUCTIONS,
      tools: [mcpToTool(mcpClient)],
    },
  });
}

function camelCaseToDash(str: string): string {
  return str
    .replace(/([a-z])([A-Z])/g, '$1-$2')
    .replace(/([A-Z])([A-Z][a-z])/g, '$1-$2')
    .toLowerCase();
}

document.addEventListener('DOMContentLoaded', async (event) => {
  const rootElement = document.querySelector('#root')! as HTMLElement;

  const playground = document.createElement('gdm-playground') as Playground;
  rootElement.appendChild(playground);

  playground.renderMapQuery({ location: 'Rome' });


  // ---------

  const [transportA, transportB] = InMemoryTransport.createLinkedPair();

  void startMcpGoogleMapServer(
    transportA,
    (
      params: {
        location?: string;
        search?: string;
        country?: string;
        city?: string;
        restaurantSearchQuery?: string;
      }
    ) => {
      playground.renderMapQuery(params);
    }
  );

  const mcpClient = await startClient(transportB);

  // --------

  const aiChat = createAiChat(mcpClient);

  playground.sendMessageHandler = async (
    input: string,
    role: string,
  ) => {
    console.log(
      'sendMessageHandler',
      input,
      role
    );

    const { thinking, text } = playground.addMessage('assistant', '');
    const message = [];

    message.push({
      role,
      text: input,
    });

    playground.setChatState(ChatState.GENERATING);

    text.innerHTML = '...';

    let newCode = '';
    let thought = '';


    try {
      const res = await aiChat.sendMessageStream({ message });

      for await (const chunk of res) {
        for (const candidate of chunk.candidates ?? []) {
          for (const part of candidate.content?.parts ?? []) {
            if (part.functionCall) {
              console.log('FUNCTION CALL:', part.functionCall.name, part.functionCall.args);
              const mcpCall = {
                name: camelCaseToDash(part.functionCall.name!),
                arguments: part.functionCall.args
              };
              // ==================================
              // Show the function call in the chat
              // ================================== 
              const explanation =
                'Calling function:\n```json\n' +
                JSON.stringify(mcpCall, null, 2);
              const { thinking, text } = playground.addMessage('assistant', '');

              text.innerHTML = await marked.parse(explanation);
            }

            if (part.thought) {
              playground.setChatState(ChatState.THINKING);
              if (part.text) {
                thought += part.text;
                thinking.innerHTML = await marked.parse(thought);
                thinking.parentElement!.classList.remove('hidden');
              }
            } else if (part.text) {
              playground.setChatState(ChatState.EXECUTING);
              newCode += part.text;
              text.innerHTML = await marked.parse(newCode);
            }
            playground.scrollToTheEnd();
          }
        }
      }
    } catch (e: any) {
      console.error('GenAI SDK Error:', e.message);
      let message = e.message;
      const splitPos = e.message.indexOf('{');
      if (splitPos > -1) {
        const msgJson = e.message.substring(splitPos);
        try {
          const sdkError = JSON.parse(msgJson);
          if (sdkError.error) {
            message = sdkError.error.message;
            message = await marked.parse(message);
          }
        } catch (e) {
          console.error('Unable to parse the error message:', e);
        }
      }
      const { text } = playground.addMessage('error', '');
      text.innerHTML = message;
    }

    // close thinking block
    thinking.parentElement!.removeAttribute('open');

    // If the answer was just code
    if (text.innerHTML.trim().length === 0) {
      text.innerHTML = 'Done';
    }

    playground.setChatState(ChatState.IDLE);
  };

  playground.setInputField(
    EXAMPLE_PROMPTS[Math.floor(Math.random() * EXAMPLE_PROMPTS.length)],
  );
});