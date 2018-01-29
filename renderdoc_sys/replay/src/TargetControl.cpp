#include "../../renderdoc/renderdoc/api/replay/renderdoc_replay.h"

#include "../include/TargetControl.h"

TargetControl::TargetControl(const char *host, uint32_t ident,
                             const char *clientName, bool forceConnection)
{
    this->inner = RENDERDOC_CreateTargetControl(host, ident, clientName, forceConnection);
}

TargetControl::~TargetControl() {
    this->Shutdown();
}

bool TargetControl::Connected() {
    return this->inner->Connected();
}

const char *TargetControl::GetTarget() {
    return this->inner->GetTarget();
}

const char *TargetControl::GetAPI() {
    return this->inner->GetAPI();
}

uint32_t TargetControl::GetPID() {
    return this->inner->GetPID();
}

const char *TargetControl::GetBusyClient() {
    return this->inner->GetBusyClient();
}

void TargetControl::TriggerCapture(uint32_t numFrames) {
    return this->inner->TriggerCapture(numFrames);
}

void TargetControl::QueueCapture(uint32_t frameNumber) {
    this->inner->QueueCapture(frameNumber);
}

void TargetControl::CopyCapture(uint32_t remoteID, const char *localpath) {
    this->inner->CopyCapture(remoteID, localpath);
}

void TargetControl::DeleteCapture(uint32_t remoteID) {
    this->inner->DeleteCapture(remoteID);
}

TargetControlMessage TargetControl::ReceiveMessage() {
    return this->inner->ReceiveMessage();
}

uint32_t TargetControl::EnumerateRemoteTargets(const char *host, uint32_t nextIdent) {
    return RENDERDOC_EnumerateRemoteTargets(host, nextIdent);
}

void TargetControl::Shutdown() {
    this->inner->Shutdown();
}
