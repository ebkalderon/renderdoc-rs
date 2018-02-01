#include "../../renderdoc/renderdoc/api/replay/renderdoc_replay.h"

#include "../include/camera.h"

Camera::Camera(ICamera *inner) {
    this->inner = inner;
}

void Camera::Shutdown() {
    this->inner->Shutdown();
}

void Camera::SetPosition(float x, float y, float z) {
    this->inner->SetPosition(x, y, z);
}

void Camera::SetFPSRotation(float x, float y, float z) {
    this->inner->SetFPSRotation(x, y, z);
}

void Camera::SetArcballDistance(float dist) {
    this->inner->SetArcballDistance(dist);
}

void Camera::ResetArcball() {
    this->inner->ResetArcball();
}

void Camera::RotateArcball(float ax, float ay, float bx, float by) {
    this->inner->RotateArcball(ax, ay, bx, by);
}

FloatVector Camera::GetPosition() {
    return this->inner->GetPosition();
}

FloatVector Camera::GetForward() {
    return this->inner->GetForward();
}

FloatVector Camera::GetRight() {
    return this->inner->GetRight();
}

FloatVector Camera::GetUp() {
    return this->inner->GetUp();
}
