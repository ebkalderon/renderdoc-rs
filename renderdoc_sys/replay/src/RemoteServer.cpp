#include "../../renderdoc/renderdoc/api/replay/renderdoc_replay.h"

#include "../include/RemoteServer.h"
#include "../include/ReplayController.h"

RemoteServer::RemoteServer(IRemoteServer *inner) {
    this->inner = inner;
}

void RemoteServer::ShutdownServerAndConnection() {
    this->inner->ShutdownServerAndConnection();
}

void RemoteServer::ShutdownConnection() {
    this->inner->ShutdownConnection();
}

bool RemoteServer::Ping() {
    return this->inner->Ping();
}

rdctype::array<rdctype::str> RemoteServer::LocalProxies() {
    return this->inner->LocalProxies();
}

rdctype::array<rdctype::str> RemoteServer::RemoteSupportedReplays() {
    return this->inner->RemoteSupportedReplays();
}

rdctype::str RemoteServer::GetHomeFolder() {
    return this->inner->GetHomeFolder();
}

rdctype::array<PathEntry> RemoteServer::ListFolder(const char *path) {
    return this->inner->ListFolder(path);
}

void RemoteServer::TakeOwnershipCapture(const char *filename) {
    this->inner->TakeOwnershipCapture(filename);
}

rdctype::str RemoteServer::CopyCaptureToRemote(const char *filename,
                                               float *progress)
{
    return this->inner->CopyCaptureToRemote(filename, progress);
}

void RemoteServer::CopyCaptureFromRemote(
    const char *remotepath,
    const char *localpath,
    float *progress
) {
    this->inner->CopyCaptureFromRemote(remotepath, localpath, progress);
}

rdctype::pair<ReplayStatus, ReplayController*> RemoteServer::OpenCapture(
    uint32_t proxyid,
    const char *logfile,
    float *progress
) {
    auto result = this->inner->OpenCapture(proxyid, logfile, progress);

    if (result.first == ReplayStatus::Succeeded) {
        ReplayController *ctrl = new ReplayController;
        ctrl->inner = result.second;
        return { result.first, ctrl };
    }

    return { result.first, NULL };
}

void RemoteServer::CloseCapture(ReplayController *ctrl) {
    this->inner->CloseCapture(ctrl->inner);
}
