#include "../../renderdoc/renderdoc/api/replay/renderdoc_replay.h"

#include "../include/CaptureFile.h"
#include "../include/ReplayController.h"

CaptureFile::CaptureFile(const char *logfile) {
    this->inner = RENDERDOC_OpenCaptureFile(logfile);
}

CaptureFile::~CaptureFile() {
    this->Shutdown();
}

ReplayStatus CaptureFile::OpenStatus() {
    return this->inner->OpenStatus();
}

const char *CaptureFile::Filename() {
    return this->inner->Filename();
}

ReplaySupport CaptureFile::LocalReplaySupport() {
    return this->inner->LocalReplaySupport();
}

const char *CaptureFile::DriverName() {
    return this->inner->DriverName();
}

const char *CaptureFile::RecordedMachineIdent() {
    return this->inner->RecordedMachineIdent();
}

rdctype::pair<ReplayStatus, ReplayController> CaptureFile::OpenCapture(float *progress) {
    auto result = this->inner->OpenCapture(progress);
    if (result.first == ReplayStatus::Succeeded) {
        
    }
}

rdctype::array<byte> CaptureFile::GetThumbnail(FileType type, uint32_t maxsize) {
    return this->inner->GetThumbnail(type, maxsize);
}

void CaptureFile::Shutdown() {
    this->inner->Shutdown();
}
