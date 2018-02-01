#include "../../renderdoc/renderdoc/api/replay/renderdoc_replay.h"

#include "../include/replay_output.h"

void ReplayOutput::SetTextureDisplay(const TextureDisplay &o) {
    this->inner->SetTextureDisplay(o);
}

void ReplayOutput::SetMeshDisplay(const MeshDisplay &o) {
    this->inner->SetMeshDisplay(o);
}

void ReplayOutput::ClearThumbnails() {
    this->inner->ClearThumbnails();
}
bool ReplayOutput::AddThumbnail(WindowingSystem system, void *data,
                                ResourceId texID, CompType typeHint)
{
    return this->inner->AddThumbnail(system, data, texID, typeHint);
}

void ReplayOutput::Display() {
    this->inner->Display();
}

bool ReplayOutput::SetPixelContext(WindowingSystem system, void *data) {
    return this->inner->SetPixelContext(system, data);
}

void ReplayOutput::SetPixelContextLocation(uint32_t x, uint32_t y) {
    this->inner->SetPixelContextLocation(x, y);
}

void ReplayOutput::DisablePixelContext() {
    this->inner->DisablePixelContext();
}

rdctype::pair<PixelValue, PixelValue> ReplayOutput::GetMinMax() {
    return this->inner->GetMinMax();
}

rdctype::array<uint32_t> ReplayOutput::GetHistogram(float minval, float maxval,
                                                    bool channels[4])
{
    return this->inner->GetHistogram(minval, maxval, channels);
}

ResourceId ReplayOutput::GetCustomShaderTexID() {
    return this->inner->GetCustomShaderTexID();
}

ResourceId ReplayOutput::GetDebugOverlayTexID() {
    return this->inner->GetDebugOverlayTexID();
}

PixelValue ReplayOutput::PickPixel(
    ResourceId texID,
    bool customShader,
    uint32_t x,
    uint32_t y,
    uint32_t sliceFace,
    uint32_t mip,
    uint32_t sample
) {
    return this->inner->PickPixel(texID, customShader, x, y, sliceFace, mip, sample);
}

rdctype::pair<uint32_t, uint32_t> ReplayOutput::PickVertex(
    uint32_t eventID,
    uint32_t x,
    uint32_t y
) {
    return this->inner->PickVertex(eventID, x, y);
}
