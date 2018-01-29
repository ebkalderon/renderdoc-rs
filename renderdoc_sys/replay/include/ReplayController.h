#ifndef REPLAY_CONTROLLER_H
#define REPLAY_CONTROLLER_H

#include <cstdint>

#include "../../renderdoc/renderdoc/api/replay/control_types.h"

class ReplayOutput;

class ReplayController {
public:
    APIProperties GetAPIProperties();
    rdctype::array<WindowingSystem> GetSupportedWindowSystems();

    ReplayOutput *CreateOutput(WindowingSystem system, void *data,
                               ReplayOutputType type);

    void Shutdown();
    void ShutdownOutput(ReplayOutput *output);

    void ReplayLoop(WindowingSystem system, void *data, ResourceId texid);
    void CancelReplayLoop();

    void FileChanged();
    bool HasCallstacks();

    bool InitResolver();

    void SetFrameEvent(uint32_t eventID, bool force);

    D3D11Pipe::State GetD3D11PipelineState();
    D3D12Pipe::State GetD3D12PipelineState();
    GLPipe::State GetGLPipelineState();
    VKPipe::State GetVulkanPipelineState();

    rdctype::array<rdctype::str> GetDisassemblyTargets();
    rdctype::str DisassembleShader(const ShaderReflection *refl,
                                   const char *target);

    rdctype::pair<ResourceId, rdctype::str> BuildCustomShader(
        const char *entry,
        const char *source,
        const uint32_t compileFlags,
        ShaderStage type
    );
    void FreeCustomShader(ResourceId id);
    rdctype::pair<ResourceId, rdctype::str> BuildTargetShader(
        const char *entry,
        const char *source,
        const uint32_t compileFlags,
        ShaderStage type
    );

    void ReplaceResource(ResourceId original, ResourceId replacement);
    void RemoveReplacement(ResourceId id);
    void FreeTargetResource(ResourceId id);

    FrameDescription GetFrameInfo();
    rdctype::array<DrawcallDescription> GetDrawcalls();

    rdctype::array<CounterResult> FetchCounters(
        const rdctype::array<GPUCounter> &counters
    );
    rdctype::array<GPUCounter> EnumerateCounters();
    CounterDescription DescribeCounter(GPUCounter counterID);

    rdctype::array<TextureDescription> GetTextures();
    rdctype::array<BufferDescription> GetBuffers();
    rdctype::array<rdctype::str> GetResolve(
        const rdctype::array<uint64_t> &callstack
    );
    rdctype::array<DebugMessage> GetDebugMessages();

    rdctype::array<PixelModification> PixelHistory(
        ResourceId texture,
        uint32_t x,
        uint32_t y,
        uint32_t slice,
        uint32_t mip,
        uint32_t sampleIdx,
        CompType typeHint
    );

    ShaderDebugTrace *DebugVertex(uint32_t vertid, uint32_t instid, uint32_t idx,
                                  uint32_t instOffset, uint32_t vertOffset);
    ShaderDebugTrace *DebugPixel(uint32_t x, uint32_t y, uint32_t sample,
                                 uint32_t primitive);
    ShaderDebugTrace *DebugThread(const uint32_t groupid[3],
                                  const uint32_t threadid[3]);
    void FreeTrace(ShaderDebugTrace *trace);

    rdctype::array<EventUsage> GetUsage(ResourceId id);
    rdctype::array<ShaderVariable> GetCBufferVariableContents(
        ResourceId shader,
        const char *entryPoint,
        uint32_t cbufslot,
        ResourceId buffer,
        uint64_t offs
    );

    bool SaveTexture(const TextureSave &saveData, const char *path);

    MeshFormat GetPostVSData(uint32_t instID, MeshDataStage stage);

    rdctype::array<byte> GetBufferData(ResourceId buff, uint64_t offset,
                                       uint64_t len);
    rdctype::array<byte> GetTextureData(ResourceId tex, uint32_t arrayIdx,
                                        uint32_t mip);

    static const uint32_t NoPreference = ~0U;

protected:
    ReplayController();
    ~ReplayController() = default;

private:
    friend class CaptureFile;
    friend class RemoteServer;

    class IReplayController *inner;
};

#endif // REPLAY_CONTROLLER_H
