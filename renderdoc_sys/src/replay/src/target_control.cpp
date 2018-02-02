#include "../../renderdoc/renderdoc/api/replay/renderdoc_replay.h"

#include "../include/target_control.h"

TargetControl::TargetControl(ITargetControl *inner) {
    this->inner = inner;
}

void TargetControl::Shutdown() {
    this->inner->Shutdown();
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
