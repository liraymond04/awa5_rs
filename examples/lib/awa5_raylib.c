#include "raylib.h"

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_MODELS 99999
static Model models[MAX_MODELS];

#define MAX_TEXURES 99999
static Texture2D textures[MAX_TEXURES];

static Camera3D camera = { 0 };

void initwindow(const uint8_t *data, uint8_t **out, size_t *out_len) {
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

void settargetfps(const uint8_t *data, uint8_t **out, size_t *out_len) {
    int32_t fps;

    memcpy(&fps, data, sizeof(int32_t));
    data += sizeof(int32_t);

    SetTargetFPS(fps);
}

void clearbackground(const uint8_t *data, uint8_t **out, size_t *out_len) {
    int32_t r, g, b;

    memcpy(&r, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&g, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&b, data, sizeof(int32_t));
    data += sizeof(int32_t);

    Color color = { r, g, b, 255 };

    ClearBackground(color);
}

void drawtext(const uint8_t *data, uint8_t **out, size_t *out_len) {
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

    Color color = { r, g, b, 255 };

    DrawText(str, posx, posy, fontsize, color);
}

void iskeydown(const uint8_t *data, uint8_t **out, size_t *out_len) {
    int32_t key;

    memcpy(&key, data, sizeof(int32_t));
    data += sizeof(int32_t);

    uint8_t ret = IsKeyDown(key);

    *out = (uint8_t *)malloc(sizeof(uint8_t));
    (*out)[0] = ret;
    *out_len = sizeof(uint8_t);
}

void drawcircle(const uint8_t *data, uint8_t **out, size_t *out_len) {
    int32_t centerx, centery;
    float radius;
    int32_t r, g, b;

    memcpy(&centerx, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&centery, data, sizeof(int32_t));
    data += sizeof(int32_t);

    memcpy(&radius, data, sizeof(float));
    data += sizeof(float);

    memcpy(&r, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&g, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&b, data, sizeof(int32_t));
    data += sizeof(int32_t);

    Color color = { r, g, b, 255 };

    DrawCircle(centerx, centery, radius, color);
}

// [in] x, y, z
void setcameraposition(const uint8_t *data, uint8_t **out, size_t *out_len) {
    float x, y, z;

    memcpy(&x, data, sizeof(float));
    data += sizeof(float);
    memcpy(&y, data, sizeof(float));
    data += sizeof(float);
    memcpy(&z, data, sizeof(float));
    data += sizeof(float);

    camera.position = (Vector3){ x, y, z };
}

// [in] x, y, z
void setcameratarget(const uint8_t *data, uint8_t **out, size_t *out_len) {
    float x, y, z;

    memcpy(&x, data, sizeof(float));
    data += sizeof(float);
    memcpy(&y, data, sizeof(float));
    data += sizeof(float);
    memcpy(&z, data, sizeof(float));
    data += sizeof(float);

    camera.target = (Vector3){ x, y, z };
}

// [in] x, y, z
void setcameraup(const uint8_t *data, uint8_t **out, size_t *out_len) {
    float x, y, z;

    memcpy(&x, data, sizeof(float));
    data += sizeof(float);
    memcpy(&y, data, sizeof(float));
    data += sizeof(float);
    memcpy(&z, data, sizeof(float));
    data += sizeof(float);

    camera.up = (Vector3){ x, y, z };
}

// [in] fovy
void setcamerafovy(const uint8_t *data, uint8_t **out, size_t *out_len) {
    float fovy;

    memcpy(&fovy, data, sizeof(float));
    data += sizeof(float);

    camera.fovy = fovy;
}

// [in] projection
void setcameraprojection(const uint8_t *data, uint8_t **out, size_t *out_len) {
    CameraProjection projection;

    memcpy(&projection, data, sizeof(CameraProjection));
    data += sizeof(CameraProjection);

    camera.projection = projection;
}

void beginmode3d(const uint8_t *data, uint8_t **out, size_t *out_len) {
    BeginMode3D(camera);
}

void drawcube(const uint8_t *data, uint8_t **out, size_t *out_len) {
    float x, y, z;

    memcpy(&x, data, sizeof(float));
    data += sizeof(float);
    memcpy(&y, data, sizeof(float));
    data += sizeof(float);
    memcpy(&z, data, sizeof(float));
    data += sizeof(float);

    Vector3 position = (Vector3){ x, y, z };

    float width, height, length;

    memcpy(&width, data, sizeof(float));
    data += sizeof(float);
    memcpy(&height, data, sizeof(float));
    data += sizeof(float);
    memcpy(&length, data, sizeof(float));
    data += sizeof(float);

    int32_t r, g, b;

    memcpy(&r, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&g, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&b, data, sizeof(int32_t));
    data += sizeof(int32_t);

    Color color = { r, g, b, 255 };

    DrawCube(position, width, height, length, color);
}

void drawcubewires(const uint8_t *data, uint8_t **out, size_t *out_len) {
    float x, y, z;

    memcpy(&x, data, sizeof(float));
    data += sizeof(float);
    memcpy(&y, data, sizeof(float));
    data += sizeof(float);
    memcpy(&z, data, sizeof(float));
    data += sizeof(float);

    Vector3 position = (Vector3){ x, y, z };

    float width, height, length;

    memcpy(&width, data, sizeof(float));
    data += sizeof(float);
    memcpy(&height, data, sizeof(float));
    data += sizeof(float);
    memcpy(&length, data, sizeof(float));
    data += sizeof(float);

    int32_t r, g, b;

    memcpy(&r, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&g, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&b, data, sizeof(int32_t));
    data += sizeof(int32_t);

    Color color = { r, g, b, 255 };

    DrawCubeWires(position, width, height, length, color);
}

void drawgrid(const uint8_t *data, uint8_t **out, size_t *out_len) {
    int32_t slices;
    float spacing;

    memcpy(&slices, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&spacing, data, sizeof(float));
    data += sizeof(float);

    DrawGrid(slices, spacing);
}

// [in] model_id, filename
void loadmodel(const uint8_t *data, uint8_t **out, size_t *out_len) {
    int32_t model_id;
    char filename[256];

    memcpy(&model_id, data, sizeof(int32_t));
    data += sizeof(int32_t);

    strcpy(filename, (char *)data);
    data += strlen(filename) + 1;

    models[model_id] = LoadModel(filename);
}

// [in] model_id
void unloadmodel(const uint8_t *data, uint8_t **out, size_t *out_len) {
    int32_t model_id;

    memcpy(&model_id, data, sizeof(int32_t));
    data += sizeof(int32_t);

    UnloadModel(models[model_id]);
}

void drawmodel(const uint8_t *data, uint8_t **out, size_t *out_len) {
    int32_t model_id;

    memcpy(&model_id, data, sizeof(int32_t));
    data += sizeof(int32_t);

    float x, y, z;

    memcpy(&x, data, sizeof(float));
    data += sizeof(float);
    memcpy(&y, data, sizeof(float));
    data += sizeof(float);
    memcpy(&z, data, sizeof(float));
    data += sizeof(float);

    Vector3 position = (Vector3){ x, y, z };

    float scale;

    memcpy(&scale, data, sizeof(float));
    data += sizeof(float);

    int32_t r, g, b;

    memcpy(&r, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&g, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&b, data, sizeof(int32_t));
    data += sizeof(int32_t);

    Color tint = (Color){ r, g, b, 255 };

    DrawModel(models[model_id], position, scale, tint);
}

void drawmodelex(const uint8_t *data, uint8_t **out, size_t *out_len) {
    int32_t model_id;

    memcpy(&model_id, data, sizeof(int32_t));
    data += sizeof(int32_t);

    float x, y, z;

    memcpy(&x, data, sizeof(float));
    data += sizeof(float);
    memcpy(&y, data, sizeof(float));
    data += sizeof(float);
    memcpy(&z, data, sizeof(float));
    data += sizeof(float);

    Vector3 position = (Vector3){ x, y, z };

    memcpy(&x, data, sizeof(float));
    data += sizeof(float);
    memcpy(&y, data, sizeof(float));
    data += sizeof(float);
    memcpy(&z, data, sizeof(float));
    data += sizeof(float);

    Vector3 rotationaxis = (Vector3){ x, y, z };

    float rotationangle;

    memcpy(&rotationangle, data, sizeof(float));
    data += sizeof(float);

    memcpy(&x, data, sizeof(float));
    data += sizeof(float);
    memcpy(&y, data, sizeof(float));
    data += sizeof(float);
    memcpy(&z, data, sizeof(float));
    data += sizeof(float);

    Vector3 scale = (Vector3){ x, y, z };

    int32_t r, g, b;

    memcpy(&r, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&g, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&b, data, sizeof(int32_t));
    data += sizeof(int32_t);

    Color tint = (Color){ r, g, b, 255 };

    DrawModelEx(models[model_id], position, rotationaxis, rotationangle, scale,
                tint);
}

void loadtexture(const uint8_t *data, uint8_t **out, size_t *out_len) {
    int32_t texture_id;
    char filename[256];

    memcpy(&texture_id, data, sizeof(int32_t));
    data += sizeof(int32_t);

    strcpy(filename, (char *)data);
    data += strlen(filename);

    textures[texture_id] = LoadTexture(filename);
}

// [in] model_id, material_index, material_map_index, texture_id
void setmaterialtexture(const uint8_t *data, uint8_t **out, size_t *out_len) {
    int32_t model_id, material_index;
    int32_t material_map_index;
    int32_t texture_id;

    memcpy(&model_id, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&material_index, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&material_map_index, data, sizeof(int32_t));
    data += sizeof(int32_t);
    memcpy(&texture_id, data, sizeof(int32_t));
    data += sizeof(int32_t);

    SetMaterialTexture(&models[model_id].materials[material_index],
                       material_map_index, textures[texture_id]);
}

void addfloat(const uint8_t *data, uint8_t **out, size_t *out_len) {
    float a, b;

    memcpy(&a, data, sizeof(float));
    data += sizeof(float);
    memcpy(&b, data, sizeof(float));
    data += sizeof(float);

    float ret = a + b;

    *out = (uint8_t *)malloc(sizeof(float));
    memcpy(*out, &ret, sizeof(float));
    *out_len = sizeof(float);
}
