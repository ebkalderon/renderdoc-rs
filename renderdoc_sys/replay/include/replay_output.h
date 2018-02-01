#ifndef REPLAY_OUTPUT_H
#define REPLAY_OUTPUT_H

#include "../../renderdoc/renderdoc/api/replay/control_types.h"

class ReplayOutput {
public:
    void SetTextureDisplay(const TextureDisplay &o);
    void SetMeshDisplay(const MeshDisplay &o);

    void ClearThumbnails();
    bool AddThumbnail(WindowingSystem system, void *data, ResourceId texID,
                      CompType typeHint);

    void Display();

    bool SetPixelContext(WindowingSystem system, void *data);
    void SetPixelContextLocation(uint32_t x, uint32_t y);
    void DisablePixelContext();

    rdctype::pair<PixelValue, PixelValue> GetMinMax();
    rdctype::array<uint32_t> GetHistogram(float minval, float maxval,
                                          bool channels[4]);

    ResourceId GetCustomShaderTexID();
    ResourceId GetDebugOverlayTexID();

    PixelValue PickPixel(ResourceId texID, bool customShader, uint32_t x,
                         uint32_t y, uint32_t sliceFace, uint32_t mip,
                         uint32_t sample);

    rdctype::pair<uint32_t, uint32_t> PickVertex(uint32_t eventID, uint32_t x,
                                                 uint32_t y);

    static const uint32_t NoResult = ~0U;

protected:
    friend class ReplayController;

    ReplayOutput() = default;
    ~ReplayOutput() = default;

private:
    class IReplayOutput *inner;
};

#endif // REPLAY_OUTPUT_H
