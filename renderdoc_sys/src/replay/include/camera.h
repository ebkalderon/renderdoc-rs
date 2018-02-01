#ifndef CAMERA_H
#define CAMERA_H

#include <cstdint>

class ICamera;
enum class CameraType : uint32_t;
struct FloatVector;

class Camera {
public:
    Camera(ICamera *inner);

    void Shutdown();

    void SetPosition(float x, float y, float z);
    void SetFPSRotation(float x, float y, float z);

    void SetArcballDistance(float dist);
    void ResetArcball();
    void RotateArcball(float ax, float ay, float bx, float by);

    FloatVector GetPosition();
    FloatVector GetForward();
    FloatVector GetRight();
    FloatVector GetUp();

private:
    ICamera *inner;
};

#endif // CAMERA_H
