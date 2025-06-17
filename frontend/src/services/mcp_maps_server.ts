/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

// tslint:disable
import {McpServer} from '@modelcontextprotocol/sdk/server/mcp.js';
import {Transport} from '@modelcontextprotocol/sdk/shared/transport.js';
import {ResourceTemplate} from "@modelcontextprotocol/sdk/server/mcp.js";
import {z} from 'zod';

export interface MapParams {
  location?: string;
  search?: string;
  country?: string;
  city?: string;
  restaurantSearchQuery?: string;
}

export async function startMcpGoogleMapServer(
  transport: Transport,
  mapQueryHandler: (params: MapParams) => void,
) {
  // Create an MCP server
  const server = new McpServer({
    name: 'Itaca App',
    version: '1.0.0',
  });

  // server.tool(
  //   'view_location_google_maps',
  //   'View a specific query or geographical location and display in the embedded maps interface',
  //   {query: z.string()},
  //   async ({query}) => {
  //     mapQueryHandler({location: query});
  //     return {
  //       content: [{type: 'text', text: `Navigating to: ${query}`}],
  //     };
  //   },
  // );

  // server.tool(
  //   'search_google_maps',
  //   'Search google maps for a series of places near a location and display it in the maps interface',
  //   {search: z.string()},
  //   async ({search}) => {
  //     mapQueryHandler({search});
  //     return {
  //       content: [{type: 'text', text: `Searching: ${search}`}],
  //     };
  //   },
  // );

  // server.tool(
  //   'directions_on_google_maps',
  //   'Search google maps for directions from origin to destination.',
  //   {origin: z.string(), destination: z.string()},
  //   async ({origin, destination}) => {
  //     mapQueryHandler({origin, destination});
  //     return {
  //       content: [
  //         {type: 'text', text: `Navigating from ${origin} to ${destination}`},
  //       ],
  //     };
  //   },
  // );

  server.tool(
    'short_term_rental_locations',
    'Searches for short-term rental locations based on a query. Can be used to find types of accommodations, specific rental names, or rentals in an area.',
    {search: z.string(), country: z.string(), city: z.string().optional()},
    async ({search}, _extra) => {
      try {
          const response = await fetch('https://db-endpoint-1019063081317.us-central1.run.app/health', {
              method: 'GET'
          });
          const data = await response.json();
          return {
              content: [{type: "text" as const, text: `Searching result: ${data.status}`}],
          };
      } catch (error) {
          return {
              content: [{type: "text" as const, text: 'Search fail.'}],
          };
      }
    }
  );

  // Tool to search for restaurants in a sample MongoDB database
  // Note: This is a placeholder. Actual MongoDB integration should be done on the backend.

  server.tool(
    'search_restaurants',
    'Searches the sample_restaurants database for restaurants based on a query. Can be used to find types of food, specific restaurant names, or restaurants in an area.',
    {restaurantSearchQuery: z.string()},
    async ({restaurantSearchQuery}, _extra) => {
      try {
        mapQueryHandler({restaurantSearchQuery});
        return {
          content: [{type: "text" as const, text: `Searching: ${restaurantSearchQuery}`}],
        };
      } catch (error) {
        return {
          content: [{type: "text" as const, text: 'Search fail.'}],
        };
      }
    },
  );

  await server.connect(transport);
  console.log('server running');
  while (true) {
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }
}