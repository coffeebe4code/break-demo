#include <SDL3/SDL_surface.h>
#define SDL_MAIN_USE_CALLBACKS 1
#include <SDL3/SDL.h>
#include <SDL3/SDL_main.h>
#include <stdio.h>

static SDL_Window *window = NULL;
static SDL_Renderer *renderer = NULL;
static SDL_Surface *surface = NULL;

/* This function runs once at startup. */
SDL_AppResult SDL_AppInit(void **appstate, int argc, char *argv[]) {
  (void)appstate;
  (void)argc;
  (void)argv;
  SDL_SetAppMetadata("Break! Towers", "1.0", "com.coffeeb4code.break-towers");

  if (!SDL_Init(SDL_INIT_VIDEO)) {
    SDL_Log("Couldn't initialize SDL: %s", SDL_GetError());
    return SDL_APP_FAILURE;
  }

  if (!SDL_CreateWindowAndRenderer("Break! Towers", 640, 480, 0, &window,
                                   &renderer)) {
    SDL_Log("Couldn't create window/renderer: %s", SDL_GetError());
    return SDL_APP_FAILURE;
  }

  surface = NULL;

  return SDL_APP_CONTINUE;
}

/* This function runs when a new event (mouse input, keypresses, etc) occurs. */
SDL_AppResult SDL_AppEvent(void *appstate, SDL_Event *event) {
  (void)appstate;

  if (event->type == SDL_EVENT_QUIT) {
    return SDL_APP_SUCCESS;
  }
  return SDL_APP_CONTINUE;
}

/* This function runs once per frame, and is the heart of the program. */
SDL_AppResult SDL_AppIterate(void *appstate) {
  (void)appstate;

  const double now = ((double)SDL_GetTicks()) / 1000.0;
  const float red = (float)(0.5 + 0.5 * SDL_sin(now));
  const float green = (float)(0.5 + 0.5 * SDL_sin(now + SDL_PI_D * 2 / 3));
  const float blue = (float)(0.5 + 0.5 * SDL_sin(now + SDL_PI_D * 4 / 3));
  SDL_SetRenderDrawColorFloat(
      renderer, red, green, blue,
      SDL_ALPHA_OPAQUE_FLOAT); /* new color, full alpha. */

  /* clear the window to the draw color. */
  SDL_RenderClear(renderer);

  /* put the newly-cleared rendering on the screen. */
  SDL_RenderPresent(renderer);

  return SDL_APP_CONTINUE; /* carry on with the program! */
}

/* This function runs once at shutdown. */
void SDL_AppQuit(void *appstate, SDL_AppResult result) {
  (void)appstate;
  (void)result;
  /* SDL will clean up the window/renderer for us. */
}
