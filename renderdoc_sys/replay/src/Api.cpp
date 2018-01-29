#include "../renderdoc/renderdoc/api/replay/renderdoc_replay.h"

#include "../include/Api.h"

#include "../include/Camera.h"
#include "../include/CaptureFile.h"
#include "../include/RemoteServer.h"
#include "../include/ReplayController.h"
#include "../include/ReplayOutput.h"
#include "../include/TargetControl.h"

Camera *RENDERDOC::InitCamera(CameraType type) {
    return new Camera(type);
}

CaptureFile *RENDERDOC::OpenCaptureFile(const char *logfile) {
    ICaptureFile *cap = RENDERDOC_OpenCaptureFile(logfile);
    return new CaptureFile(cap);
}

TargetControl *RENDERDOC::CreateTargetControl(const char *host, uint32_t ident,
                                              const char *clientName,
                                              bool32 forceConnection)
{
    ITargetControl *ctrl = RENDERDOC_CreateTargetControl(host, ident, clientName, forceConnection);
    return new TargetControl(ctrl);
}

ReplayStatus RENDERDOC::CreateRemoteServerConnection(const char *host,
                                                     uint32_t port,
                                                     RemoteServer **rend)
{
    IRemoteServer *server = nullptr;
    ReplayStatus status = RENDERDOC_CreateRemoteServerConnection(host, port, &server);

    if (status == ReplayStatus::Succeeded) {
        *rend = new RemoteServer(server);
    }

    return status;
}
