#include "../renderdoc/renderdoc/api/replay/renderdoc_replay.h"

#include "../include/api.h"

#include "../include/camera.h"
#include "../include/capture_file.h"
#include "../include/remote_server.h"
#include "../include/replay_controller.h"
#include "../include/replay_output.h"
#include "../include/target_control.h"

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
