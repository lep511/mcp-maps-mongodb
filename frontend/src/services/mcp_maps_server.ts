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
  lat?: number;
  lng?: number;
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

  // 'Searches for short-term rental locations based on a query. Can be used to find types of accommodations, specific rental names, or rentals in an area.',

  server.tool(
    'short_term_rental_locations',
    'Search for short-term rental location based on a query. Can be used to find type of accommodation, specific rental name, or rental in an area.',
    {search: z.string(), country: z.string(), city: z.string().optional()},
    async ({search}, _extra) => {
      try {
          const get_location = await fetch('https://db-endpoint-l3l4na2aua-uc.a.run.app/mock', {
              method: 'GET'
          });
          const response = await get_location.json();
          let textResponse = response.data;
          if (typeof textResponse !== 'string') {
              textResponse = JSON.stringify(textResponse);
          }
          console.log('Response from server:', textResponse);
          const coordinates = extractCoordinates(response.data);
          if (coordinates) {
            const { longitude, latitude } = coordinates;
            console.log('Longitude:', longitude);
            console.log('Latitude:', latitude);
            mapQueryHandler({ lat: latitude, lng: longitude });
            return {
              content: [{type: "text" as const, text: `Search result: ${textResponse}`}],
            };
          } else {
            console.log('No valid coordinates found.');
            return {
              content: [{type: "text" as const, text: 'No valid coordinates found.'}],
            };
          }
      } catch (error) {
          return {
              content: [{type: "text" as const, text: 'Search fail.'}],
          };
      }
    }
  );

  // server.tool(
  //   'search_restaurants',
  //   'Searches the sample_restaurants database for restaurants based on a query. Can be used to find types of food, specific restaurant names, or restaurants in an area.',
  //   {restaurantSearchQuery: z.string()},
  //   async ({restaurantSearchQuery}, _extra) => {
  //     try {
  //       mapQueryHandler({restaurantSearchQuery});
  //       return {
  //         content: [{type: "text" as const, text: `Searching: ${restaurantSearchQuery}`}],
  //       };
  //     } catch (error) {
  //       return {
  //         content: [{type: "text" as const, text: 'Search fail.'}],
  //       };
  //     }
  //   },
  // );

  await server.connect(transport);
  console.log('server running');
  while (true) {
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }
}

// Extract longitude and latitude from response
function extractCoordinates(response: any) {
  if (response?.address?.location?.coordinates && 
      Array.isArray(response.address.location.coordinates) && 
      response.address.location.coordinates.length >= 2) {
    
    const longitude = response.address.location.coordinates[0];
    const latitude = response.address.location.coordinates[1];
    
    // Validate that coordinates are numbers
    if (typeof longitude === 'number' && typeof latitude === 'number') {
      return { longitude, latitude };
    }
  }
  
  // Return null if coordinates don't exist or are invalid
  return null;
}