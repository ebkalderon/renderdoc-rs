#include "../../renderdoc/renderdoc/api/replay/renderdoc_replay.h"

#include "../include/ReplayController.h"
#include "../include/ReplayOutput.h"

APIProperties ReplayController::GetAPIProperties() {
    return this->inner->GetAPIProperties();
}

rdctype::array<WindowingSystem> ReplayController::GetSupportedWindowSystems() {
    return this->inner->GetSupportedWindowSystems();
}

ReplayOutput *ReplayController::CreateOutput(WindowingSystem system, void *data,
                                             ReplayOutputType type)
{
    IReplayOutput *inner_out = this->inner->CreateOutput(system, data, type);
    ReplayOutput *out = new ReplayOutput;
    out->inner = inner_out;
    return out;
}

void ReplayController::Shutdown() {
    this->inner->Shutdown();
}

void ReplayController::ShutdownOutput(ReplayOutput *output) {
    this->inner->ShutdownOutput(output->inner);
    delete output;
}

void ReplayController::ReplayLoop(WindowingSystem system, void *data,
                                  ResourceId texid)
{
    this->inner->ReplayLoop(system, data, texid);
}

void ReplayController::CancelReplayLoop() {
    this->inner->CancelReplayLoop();
}

void ReplayController::FileChanged() {
    this->inner->FileChanged();
}

bool ReplayController::HasCallstacks() {
    return this->inner->HasCallstacks();
}

bool ReplayController::InitResolver() {
    return this->inner->InitResolver();
}

void ReplayController::SetFrameEvent(uint32_t eventID, bool force) {
    this->inner->SetFrameEvent(eventID, force);
}

D3D11Pipe::State ReplayController::GetD3D11PipelineState() {
    return this->inner->GetD3D11PipelineState();
}

D3D12Pipe::State ReplayController::GetD3D12PipelineState() {
    return this->inner->GetD3D12PipelineState();
}

GLPipe::State ReplayController::GetGLPipelineState() {
    return this->inner->GetGLPipelineState();
}

VKPipe::State ReplayController::GetVulkanPipelineState() {
    return this->inner->GetVulkanPipelineState();
}

rdctype::array<rdctype::str> ReplayController::GetDisassemblyTargets() {
    return this->inner->GetDisassemblyTargets();
}

rdctype::str ReplayController::DisassembleShader(const ShaderReflection *refl,
                                                 const char *target)
{
    return this->inner->DisassembleShader(refl, target);
}

rdctype::pair<ResourceId, rdctype::str> ReplayController::BuildCustomShader(
    const char *entry,
    const char *source,
    const uint32_t compileFlags,
    ShaderStage type
) {
    return this->inner->BuildCustomShader(entry, source, compileFlags, type);
}

void ReplayController::FreeCustomShader(ResourceId id) {
    this->inner->FreeCustomShader(id);
}

rdctype::pair<ResourceId, rdctype::str> ReplayController::BuildTargetShader(
    const char *entry,
    const char *source,
    const uint32_t compileFlags,
    ShaderStage type
) {
    return this->inner->BuildTargetShader(entry, source, compileFlags, type);
}

void ReplayController::ReplaceResource(ResourceId original, ResourceId replacement) {
    this->inner->ReplaceResource(original, replacement);
}

void ReplayController::RemoveReplacement(ResourceId id) {
    this->inner->RemoveReplacement(id);
}

void ReplayController::FreeTargetResource(ResourceId id) {
    this->inner->FreeTargetResource(id);
}

FrameDescription ReplayController::GetFrameInfo() {
    return this->inner->GetFrameInfo();
}

rdctype::array<DrawcallDescription> ReplayController::GetDrawcalls() {
    return this->inner->GetDrawcalls();
}

rdctype::array<CounterResult> ReplayController::FetchCounters(
    const rdctype::array<GPUCounter> &counters
) {
    return this->inner->FetchCounters(counters);
}

rdctype::array<GPUCounter> ReplayController::EnumerateCounters() {
    return this->inner->EnumerateCounters();
}

CounterDescription ReplayController::DescribeCounter(GPUCounter counterID) {
    return this->inner->DescribeCounter(counterID);
}

rdctype::array<TextureDescription> ReplayController::GetTextures() {
    return this->inner->GetTextures();
}

rdctype::array<BufferDescription> ReplayController::GetBuffers() {
    return this->inner->GetBuffers();
}

rdctype::array<rdctype::str> ReplayController::GetResolve(
    const rdctype::array<uint64_t> &callstack
) {
    return this->inner->GetResolve(callstack);
}

rdctype::array<DebugMessage> ReplayController::GetDebugMessages() {
    return this->inner->GetDebugMessages();
}

rdctype::array<PixelModification> ReplayController::PixelHistory(
    ResourceId texture,
    uint32_t x,
    uint32_t y,
    uint32_t slice,
    uint32_t mip,
    uint32_t sampleIdx,
    CompType typeHint
) {
    return this->inner->PixelHistory(texture, x, y, slice, mip, sampleIdx, typeHint);
}

ShaderDebugTrace *ReplayController::DebugVertex(
    uint32_t vertid,
    uint32_t instid,
    uint32_t idx,
    uint32_t instOffset,
    uint32_t vertOffset
) {
    return this->inner->DebugVertex(vertid, instid, idx, instOffset, vertOffset);
}

ShaderDebugTrace *ReplayController::DebugPixel(
    uint32_t x,
    uint32_t y,
    uint32_t sample,
    uint32_t primitive
) {
    return this->inner->DebugPixel(x, y, sample, primitive);
}

ShaderDebugTrace *ReplayController::DebugThread(const uint32_t groupid[3],
                                                const uint32_t threadid[3])
{
    return this->inner->DebugThread(groupid, threadid);
}

void ReplayController::FreeTrace(ShaderDebugTrace *trace) {
    this->inner->FreeTrace(trace);
}

rdctype::array<EventUsage> ReplayController::GetUsage(ResourceId id) {
    return this->inner->GetUsage(id);
}

rdctype::array<ShaderVariable> ReplayController::GetCBufferVariableContents(
    ResourceId shader,
    const char *entryPoint,
    uint32_t cbufslot,
    ResourceId buffer,
    uint64_t offs
) {
    return this->inner->GetCBufferVariableContents(shader, entryPoint, cbufslot,
                                                   buffer, offs);
}

bool ReplayController::SaveTexture(const TextureSave &saveData, const char *path) {
    return this->inner->SaveTexture(saveData, path);
}

MeshFormat ReplayController::GetPostVSData(uint32_t instID, MeshDataStage stage) {
    return this->inner->GetPostVSData(instID, stage);
}

rdctype::array<byte> ReplayController::GetBufferData(
    ResourceId buff,
    uint64_t offset,
    uint64_t len
) {
    return this->inner->GetBufferData(buff, offset, len);
}

rdctype::array<byte> ReplayController::GetTextureData(
    ResourceId tex,
    uint32_t arrayIdx,
    uint32_t mip
) {
    return this->inner->GetTextureData(tex, arrayIdx, mip);
}
