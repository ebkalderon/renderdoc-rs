#include "../../renderdoc/renderdoc/api/replay/renderdoc_replay.h"

#include "../include/RemoteServer.h"
#include "../include/ReplayController.h"

void RemoteServer::BecomeRemoteServer(
    const char *listenhost,
    uint32_t port,
    uint32_t *killReplay
) {
    RENDERDOC_BecomeRemoteServer(listenhost, port, killReplay);
}

uint32_t RemoteServer::ExecuteAndInject(
    const char *app,
    const char *workingDir,
    const char *cmdLine,
    const rdctype::array<EnvironmentModification> &env,
    const char *logfile,
    const CaptureOptions &opts,
    bool32 waitForExit
) {
    return RENDERDOC_ExecuteAndInject(
        app, workingDir, cmdLine, env, logfile, opts, waitForExit
    );
}

uint32_t RemoteServer::GetDefaultRemoteServerPort() {
    return RENDERDOC_GetDefaultRemoteServerPort();
}

RemoteServer::RemoteServer(const char *host, uint32_t port) {
    RENDERDOC_CreateRemoteServerConnection(host, port, &this->inner);
}

RemoteServer::~RemoteServer() {
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

void RemoteServer::ShutdownServerAndConnection() {
    this->inner->ShutdownServerAndConnection();
}
