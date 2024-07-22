#include "raylib.h"

#include <stdint.h>
#include <stdio.h>
#include <string.h>

void initwindow(const uint8_t *data, size_t length) {
    int32_t width, height;
    char title[256];

    memcpy(&width, data, sizeof(int32_t));
    data += sizeof(int32_t);

    memcpy(&height, data, sizeof(int32_t));
    data += sizeof(int32_t);

    strcpy(title, (char *)data);
    data += strlen(title) + 1;

    InitWindow(width, height, title);
}

void settargetfps(const uint8_t *data, size_t length) {
    int32_t fps;

    memcpy(&fps, data, sizeof(int32_t));
    data += sizeof(int32_t);

    SetTargetFPS(fps);
}

void clearbackground(const uint8_t *data, size_t length) {
    int32_t r, g, b;

    memcpy(&r, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&g, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&b, data, sizeof(int32_t));
    data += sizeof(int32_t);

    Color color = {r, g, b, 255};

    ClearBackground(color);
}

void drawtext(const uint8_t *data, size_t length) {
    char str[256];

    int32_t posx, posy;
    int32_t fontsize;
    int32_t r, g, b;

    strcpy(str, (char *)data);
    data += strlen(str) + 1;

    memcpy(&posx, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&posy, data, sizeof(int32_t));
    data += sizeof(int32_t);

    memcpy(&fontsize, data, sizeof(int32_t));
    data += sizeof(int32_t);

    memcpy(&r, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&g, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&b, data, sizeof(int32_t));
    data += sizeof(int32_t);

    Color color = {r, g, b, 255};

    DrawText(str, posx, posy, fontsize, color);
}
