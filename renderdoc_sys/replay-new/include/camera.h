#ifndef CAMERA_H
#define CAMERA_H

#ifdef __cplusplus
#define EXTERN_C extern "C" {
#define EXTERN_C_END }
#else
#define EXTERN_C
#define EXTERN_C_END
#endif

#include "float_vector.h"

EXTERN_C

enum CameraType {
    Arcball = 0,
    FPSLook,
};

typedef struct _Camera Camera;

Camera *renderdoc_camera_new(CameraType type);

void renderdoc_camera_set_position(Camera *self, float x, float y, float z);
void renderdoc_camera_set_fps_rotation(Camera *self, float x, float y, float z);

void renderdoc_camera_set_arcball_distance(Camera *self, float dist);
void renderdoc_camera_reset_arcball(Camera *self);
void renderdoc_camera_rotate_arcball(Camera *self, float old_x, float old_y,
                                     float new_x, float new_y);

FloatVector renderdoc_camera_get_position(const Camera *self);
FloatVector renderdoc_camera_get_forward(const Camera *self);
FloatVector renderdoc_camera_get_right(const Camera *self);
FloatVector renderdoc_camera_get_up(const Camera *self);

void renderdoc_camera_drop(Camera *self);

EXTERN_C_END

#endif // CAMERA_H
